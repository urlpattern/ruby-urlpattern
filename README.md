# urlpattern

[![Ruby Style Guide](https://img.shields.io/badge/code_style-rubocop-brightgreen.svg)](https://github.com/rubocop/rubocop)
[![Gem Version](https://badge.fury.io/rb/urlpattern.svg)](https://badge.fury.io/rb/urlpattern)
[![Ruby](https://github.com/urlpattern/ruby-urlpattern/actions/workflows/main.yml/badge.svg)](https://github.com/urlpattern/ruby-urlpattern/actions/workflows/main.yml)

An implementation of [the URL Pattern Standard](https://urlpattern.spec.whatwg.org/) for Ruby written in Rust.

## Description

It's a thin wrapper of [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern) with [Magnus](https://github.com/matsadler/magnus).

It is useful on the server side when serving different pages based on the URL (a.k.a. routing). It provides pattern matching syntax like `/users/:id`, similar to [route parameters in Express](https://expressjs.com/en/guide/routing.html#route-parameters) or [Path-to-RegExp](https://github.com/pillarjs/path-to-regexp). You can use it as a foundation to build your own web server or framework.

## Installation

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add urlpattern
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install urlpattern
```

## Usage

This library aims to expose an interface as close as possible to the URL Pattern Standard, but some differences are unavoidable because it is designed for Ruby, not JavaScript. For the exact details, please refer to [urlpattern.rbs](https://github.com/urlpattern/ruby-urlpattern/blob/main/sig/urlpattern.rbs).

Most JavaScript examples from [Chrome for Developers](https://developer.chrome.com/docs/web-platform/urlpattern) and [MDN](https://developer.mozilla.org/en-US/docs/Web/API/URL_Pattern_API) can be adapted to Ruby without much difficulty.

### `test`

```rb
require "urlpattern"

pattern = URLPattern::URLPattern.new "https://example.com/admin/*"
pattern.test? "https://example.com/admin/main/" #=> true
pattern.test? "https://example.com/main/"       #=> false
```

### `exec`

```rb
require "urlpattern"

pattern = URLPattern::URLPattern.new pathname: "/users/:id/"
result = pattern.exec pathname: "/users/4163/"
result[:pathname][:groups][:id] #=> 4163
```

### `base_url`

```rb
require "urlpattern"

pattern = URLPattern::URLPattern.new "b", "https://example.com/a/"
pattern.test? "a/b", "https://example.com/"                     #=> true
pattern.test? "b", "https://example.com/a/"                     #=> true
pattern.test? pathname: "b", base_url: "https://example.com/a/" #=> true
```

### `ignore_case`

```rb
require "urlpattern"

pattern = URLPattern::URLPattern.new "https://example.com/test"
pattern.test? "https://example.com/test" #=> true
pattern.test? "https://example.com/TeST" #=> false

pattern = URLPattern::URLPattern.new "https://example.com/test", ignore_case: true
pattern.test? "https://example.com/test" #=> true
pattern.test? "https://example.com/TeST" #=> true
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/urlpattern/ruby-urlpattern. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/urlpattern/ruby-urlpattern/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the Urlpattern project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/urlpattern/ruby-urlpattern/blob/main/CODE_OF_CONDUCT.md).
