use magnus::{
    Error, ExceptionClass, Module, RHash, RModule, RString, Ruby, Value, function, method,
    prelude::*, scan_args::scan_args, wrap,
};

#[wrap(class = "URLPattern::URLPattern")]
struct UrlPattern(urlpattern::UrlPattern);

impl UrlPattern {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args(args)?;
        let _: () = args.required;
        let (input, base_url, options): (Option<Value>, Option<Value>, Option<RHash>) =
            args.optional;
        let _: () = args.splat;
        let _: () = args.trailing;
        let _: () = args.keywords;
        let _: () = args.block;

        let module: RModule = ruby.class_object().const_get("URLPattern")?;
        let error_class: ExceptionClass = module.const_get("Error")?;

        let (base_url, options) = match base_url {
            Some(base_url) => {
                if base_url.is_kind_of(ruby.class_hash()) {
                    (None, Some(RHash::try_convert(base_url)?))
                } else {
                    (Some(base_url), options)
                }
            }
            None => (None, options),
        };

        let base_url = match base_url {
            Some(base_url) => Some(
                String::try_convert(base_url)?
                    .parse()
                    .map_err(|e: url::ParseError| Error::new(error_class, e.to_string()))?,
            ),
            None => None,
        };

        let options = match options {
            Some(options) => urlpattern::UrlPatternOptions {
                ignore_case: options.fetch::<_, bool>(ruby.to_symbol("ignore_case"))?,
                ..urlpattern::UrlPatternOptions::default()
            },
            None => urlpattern::UrlPatternOptions::default(),
        };

        let init: urlpattern::UrlPatternInit = match input {
            Some(input) if input.is_kind_of(ruby.class_string()) => {
                urlpattern::UrlPatternInit::parse_constructor_string::<regex::Regex>(
                    String::try_convert(input)?.as_str(),
                    base_url,
                )
                .map_err(|e| Error::new(error_class, e.to_string()))?
            }
            Some(input) if input.is_kind_of(ruby.class_hash()) => {
                if base_url.is_some() {
                    return Err(Error::new(
                        error_class,
                        "base_url cannot be provided when input is a Hash",
                    ));
                }

                let input = RHash::try_convert(input)?;

                urlpattern::UrlPatternInit {
                    protocol: input.lookup(ruby.to_symbol("protocol"))?,
                    username: input.lookup(ruby.to_symbol("username"))?,
                    password: input.lookup(ruby.to_symbol("password"))?,
                    hostname: input.lookup(ruby.to_symbol("hostname"))?,
                    port: input.lookup(ruby.to_symbol("port"))?,
                    pathname: input.lookup(ruby.to_symbol("pathname"))?,
                    search: input.lookup(ruby.to_symbol("search"))?,
                    hash: input.lookup(ruby.to_symbol("hash"))?,
                    base_url: input
                        .lookup::<_, Option<String>>(ruby.to_symbol("base_url"))?
                        .map(|s| s.parse())
                        .transpose()
                        .map_err(|e: url::ParseError| Error::new(error_class, e.to_string()))?,
                }
            }
            None => urlpattern::UrlPatternInit::default(),
            Some(_) => {
                return Err(Error::new(error_class, "invalid input type"));
            }
        };

        Ok(Self(
            urlpattern::UrlPattern::parse(init, options)
                .map_err(|e| Error::new(error_class, e.to_string()))?,
        ))
    }

    fn test(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<bool, Error> {
        let args = scan_args(args)?;
        let _: () = args.required;
        let (input, base_url): (Option<Value>, Option<String>) = args.optional;
        let _: () = args.splat;
        let _: () = args.trailing;
        let _: () = args.keywords;
        let _: () = args.block;

        let module: RModule = ruby.class_object().const_get("URLPattern")?;
        let error_class: ExceptionClass = module.const_get("Error")?;

        let input: urlpattern::UrlPatternMatchInput = match input {
            Some(input) if input.is_kind_of(ruby.class_string()) => {
                let input = String::try_convert(input)?;

                match base_url {
                    Some(base_url) => {
                        let base_url = match url::Url::parse(&base_url) {
                            Ok(url) => url,
                            Err(_) => return Ok(false),
                        };
                        ::urlpattern::UrlPatternMatchInput::Url(
                            match url::Url::options().base_url(Some(&base_url)).parse(&input) {
                                Ok(url) => url,
                                Err(_) => return Ok(false),
                            },
                        )
                    }
                    None => {
                        ::urlpattern::UrlPatternMatchInput::Url(match url::Url::parse(&input) {
                            Ok(url) => url,
                            Err(e) => return Err(Error::new(error_class, e.to_string())),
                        })
                    }
                }
            }
            Some(input) if input.is_kind_of(ruby.class_hash()) => {
                let input = RHash::try_convert(input)?;

                urlpattern::UrlPatternMatchInput::Init(urlpattern::UrlPatternInit {
                    protocol: input.lookup(ruby.to_symbol("protocol"))?,
                    username: input.lookup(ruby.to_symbol("username"))?,
                    password: input.lookup(ruby.to_symbol("password"))?,
                    hostname: input.lookup(ruby.to_symbol("hostname"))?,
                    port: input.lookup(ruby.to_symbol("port"))?,
                    pathname: input.lookup(ruby.to_symbol("pathname"))?,
                    search: input.lookup(ruby.to_symbol("search"))?,
                    hash: input.lookup(ruby.to_symbol("hash"))?,
                    base_url: input
                        .lookup::<_, Option<String>>(ruby.to_symbol("base_url"))?
                        .map(|s| s.parse())
                        .transpose()
                        .map_err(|e: url::ParseError| Error::new(error_class, e.to_string()))?,
                })
            }
            None => urlpattern::UrlPatternMatchInput::Init(urlpattern::UrlPatternInit::default()),
            Some(_) => return Err(Error::new(error_class, "invalid input type")),
        };

        rb_self
            .0
            .test(input)
            .map_err(|e| Error::new(error_class, e.to_string()))
    }

    fn exec(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Option<RHash>, Error> {
        let args = scan_args(args)?;
        let _: () = args.required;
        let (input, base_url): (Option<Value>, Option<RString>) = args.optional;
        let _: () = args.splat;
        let _: () = args.trailing;
        let _: () = args.keywords;
        let _: () = args.block;

        let module: RModule = ruby.class_object().const_get("URLPattern")?;
        let error_class: ExceptionClass = module.const_get("Error")?;

        let urlpattern_input: urlpattern::UrlPatternMatchInput = match input {
            Some(input) if input.is_kind_of(ruby.class_string()) => {
                let input = String::try_convert(input)?;

                match base_url {
                    Some(base_url) => {
                        let base_url = match url::Url::parse(&base_url.to_string()?) {
                            Ok(url) => url,
                            Err(_) => return Ok(None),
                        };
                        ::urlpattern::UrlPatternMatchInput::Url(
                            match url::Url::options().base_url(Some(&base_url)).parse(&input) {
                                Ok(url) => url,
                                Err(_) => return Ok(None),
                            },
                        )
                    }
                    None => {
                        ::urlpattern::UrlPatternMatchInput::Url(match url::Url::parse(&input) {
                            Ok(url) => url,
                            Err(e) => return Err(Error::new(error_class, e.to_string())),
                        })
                    }
                }
            }
            Some(input) if input.is_kind_of(ruby.class_hash()) => {
                let input = RHash::try_convert(input)?;

                urlpattern::UrlPatternMatchInput::Init(urlpattern::UrlPatternInit {
                    protocol: input.lookup(ruby.to_symbol("protocol"))?,
                    username: input.lookup(ruby.to_symbol("username"))?,
                    password: input.lookup(ruby.to_symbol("password"))?,
                    hostname: input.lookup(ruby.to_symbol("hostname"))?,
                    port: input.lookup(ruby.to_symbol("port"))?,
                    pathname: input.lookup(ruby.to_symbol("pathname"))?,
                    search: input.lookup(ruby.to_symbol("search"))?,
                    hash: input.lookup(ruby.to_symbol("hash"))?,
                    base_url: input
                        .lookup::<_, Option<String>>(ruby.to_symbol("base_url"))?
                        .map(|s| s.parse())
                        .transpose()
                        .map_err(|e: url::ParseError| Error::new(error_class, e.to_string()))?,
                })
            }
            None => urlpattern::UrlPatternMatchInput::Init(urlpattern::UrlPatternInit::default()),
            Some(_) => return Err(Error::new(error_class, "invalid input type")),
        };

        let Some(urlpattern_result) = rb_self
            .0
            .exec(urlpattern_input)
            .map_err(|e| Error::new(error_class, e.to_string()))?
        else {
            return Ok(None);
        };

        let result = ruby.hash_new();

        let inputs = ruby.ary_new();
        if let Some(input) = input {
            inputs.push(input)?;
        } else {
            inputs.push(ruby.hash_new())?;
        }
        if let Some(base_url) = base_url {
            inputs.push(base_url)?;
        }
        result.aset(ruby.to_symbol("inputs"), inputs)?;

        let protocol = ruby.hash_new();
        protocol.aset(ruby.to_symbol("input"), urlpattern_result.protocol.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.protocol.groups {
            groups.aset(key, value)?;
        }
        protocol.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("protocol"), protocol)?;

        let username = ruby.hash_new();
        username.aset(ruby.to_symbol("input"), urlpattern_result.username.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.username.groups {
            groups.aset(key, value)?;
        }
        username.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("username"), username)?;

        let password = ruby.hash_new();
        password.aset(ruby.to_symbol("input"), urlpattern_result.password.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.password.groups {
            groups.aset(key, value)?;
        }
        password.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("password"), password)?;

        let hostname = ruby.hash_new();
        hostname.aset(ruby.to_symbol("input"), urlpattern_result.hostname.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.hostname.groups {
            groups.aset(key, value)?;
        }
        hostname.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("hostname"), hostname)?;

        let port = ruby.hash_new();
        port.aset(ruby.to_symbol("input"), urlpattern_result.port.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.port.groups {
            groups.aset(key, value)?;
        }
        port.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("port"), port)?;

        let pathname = ruby.hash_new();
        pathname.aset(ruby.to_symbol("input"), urlpattern_result.pathname.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.pathname.groups {
            groups.aset(key, value)?;
        }
        pathname.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("pathname"), pathname)?;

        let search = ruby.hash_new();
        search.aset(ruby.to_symbol("input"), urlpattern_result.search.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.search.groups {
            groups.aset(key, value)?;
        }
        search.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("search"), search)?;

        let hash = ruby.hash_new();
        hash.aset(ruby.to_symbol("input"), urlpattern_result.hash.input)?;
        let groups = ruby.hash_new();
        for (key, value) in urlpattern_result.hash.groups {
            groups.aset(key, value)?;
        }
        hash.aset(ruby.to_symbol("groups"), groups)?;
        result.aset(ruby.to_symbol("hash"), hash)?;

        Ok(Some(result))
    }

    fn protocol(&self) -> &str {
        self.0.protocol()
    }
    fn username(&self) -> &str {
        self.0.username()
    }
    fn password(&self) -> &str {
        self.0.password()
    }
    fn hostname(&self) -> &str {
        self.0.hostname()
    }
    fn port(&self) -> &str {
        self.0.port()
    }
    fn pathname(&self) -> &str {
        self.0.pathname()
    }
    fn search(&self) -> &str {
        self.0.search()
    }
    fn hash(&self) -> &str {
        self.0.hash()
    }

    fn has_regexp_groups(&self) -> bool {
        self.0.has_regexp_groups()
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("URLPattern")?;
    let _error = module.define_error("Error", ruby.exception_standard_error())?;
    let class = module.define_class("URLPattern", ruby.class_object())?;
    class.define_singleton_method("new", function!(UrlPattern::new, -1))?;
    class.define_method("test?", method!(UrlPattern::test, -1))?;
    class.define_method("exec", method!(UrlPattern::exec, -1))?;
    class.define_method("protocol", method!(UrlPattern::protocol, 0))?;
    class.define_method("username", method!(UrlPattern::username, 0))?;
    class.define_method("password", method!(UrlPattern::password, 0))?;
    class.define_method("hostname", method!(UrlPattern::hostname, 0))?;
    class.define_method("port", method!(UrlPattern::port, 0))?;
    class.define_method("pathname", method!(UrlPattern::pathname, 0))?;
    class.define_method("search", method!(UrlPattern::search, 0))?;
    class.define_method("hash", method!(UrlPattern::hash, 0))?;
    class.define_method(
        "has_regexp_groups?",
        method!(UrlPattern::has_regexp_groups, 0),
    )?;
    Ok(())
}
