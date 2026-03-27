# frozen_string_literal: true

require "test_helper"

class TestUrlpattern < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Urlpattern::VERSION
  end

  def test_hello_world
    assert_equal "Hello earth, from Rust!", Urlpattern.hello("world")
  end
end
