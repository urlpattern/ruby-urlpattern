# frozen_string_literal: true

require "oj"
require "test_helper"

class TestURLPattern < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::URLPattern::VERSION
  end

  # This test is based on the web-platform-tests Project.
  #
  # To update the test data:
  #
  # 1. Go to https://github.com/web-platform-tests/wpt/blob/master/urlpattern/resources/urlpatterntestdata.json.
  # 2. Copy the content.
  # 3. Paste into `test/fixtures/urlpatterntestdata.json`.
  URLPATTERNTESTDATA = begin
    UNDERSCORE = { baseURL: :base_url, ignoreCase: :ignore_case }.freeze

    # Use `Oj.strict_load` with `allow_invalid_unicode` to work around `JSON::ParserError` (incomplete surrogate pair).
    Oj.strict_load(File.read(
                     File.join(__dir__, "fixtures", "urlpatterntestdata.json"), encoding: Encoding::UTF_8
                   ), { allow_invalid_unicode: true, symbol_keys: true }).map do |entry|
      entry[:pattern]&.map! { |arg| arg.is_a?(Hash) ? arg.transform_keys(UNDERSCORE) : arg }

      entry[:inputs]&.map! { |arg| arg.is_a?(Hash) ? arg.transform_keys(UNDERSCORE) : arg }

      if entry[:expected_match].is_a?(Hash)
        entry[:expected_match].transform_keys!(UNDERSCORE)
        entry[:expected_match][:inputs]&.map! { |arg| arg.is_a?(Hash) ? arg.transform_keys(UNDERSCORE) : arg }
      end

      entry[:exactly_empty_components]&.map! { |component| UNDERSCORE.fetch(component.to_sym, component.to_sym) }

      entry
    end
  end

  URLPATTERNTESTDATA.each_with_index do |entry, i|
    define_method("test_urlpattern_#{i}") do
      skip_if_unsupported entry

      return if assert_expected_obj_error entry

      pattern = new_urlpattern entry

      assert_expected_obj pattern, entry

      return if assert_expected_match pattern, entry

      assert_exactly_empty_components pattern, entry
    end
  end

  private

  def skip_if_unsupported(entry)
    skip if [[{ pathname: "*{}**?" }], ["((?R)):"]].include?(entry[:pattern])
  end

  def assert_expected_obj_error(entry)
    return unless entry[:expected_obj] == "error"

    assert_raises(StandardError) { URLPattern::URLPattern.new(*entry[:pattern]) }
    true
  end

  def new_urlpattern(entry)
    URLPattern::URLPattern.new(*entry[:pattern])
  rescue EncodingError
    skip
  end

  def assert_expected_obj(pattern, entry)
    entry[:expected_obj]&.each do |key, value|
      assert_equal pattern.send(key), value
    end
  end

  def assert_expected_match(pattern, entry)
    if entry[:expected_match] == "error"
      assert_expected_match_error pattern, entry
    elsif entry[:expected_match].is_a?(Hash)
      assert_expected_match_hash pattern, entry
    else
      assert_expected_match_nil pattern, entry
    end
  end

  def assert_expected_match_error(pattern, entry)
    assert_raises(StandardError) { pattern.test?(*entry[:inputs]) }

    assert_raises(StandardError) { pattern.exec(*entry[:inputs]) }

    true
  end

  def assert_expected_match_hash(pattern, entry)
    assert pattern.test?(*entry[:inputs])

    result = pattern.exec(*entry[:inputs])
    refute_nil result
    entry[:expected_match].each do |key, expected|
      assert_equal result[key], expected
    end
  end

  def assert_expected_match_nil(pattern, entry)
    refute pattern.test?(*entry[:inputs])

    assert_nil pattern.exec(*entry[:inputs])
  end

  def assert_exactly_empty_components(pattern, entry)
    return unless entry.key?(:exactly_empty_components)

    result = pattern.exec(*entry[:inputs])
    entry[:exactly_empty_components].each do |component|
      assert_equal(result[component][:groups], {}) if result
    end
  end
end
