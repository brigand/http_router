//! This is an abstract http router that can be used with any library, incl. Hyper, Actix, etc.
//! Usage:
//!
//! ```
//! let router = router!(request,
//!     GET /users/:user_id/widgets => users_widgets_list,
//!     POST /users/:user_id/widgets => users_widgets_create,
//! );
//!
//! router(request)
//! 
//! Working types: anything that implements FromStr. Since &str doesn't implement FromStr - use String instead.
//!
//! fn users_widgets_list(request, user_id: u32) -> impl Future<Item = (), Error = ()> {
//!     unimplemented!()
//! }
//!
//! fn users_widgets_create(request, user_id: u32) -> impl Future<Item = (), Error = ()> {
//!     unimplemented!()
//! }
//! ```

// #![feature(trace_macros)]
#[allow(unused_macros)]

extern crate regex;


#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE
}

macro_rules! router {
    // convert params from string
    (@parse_type $value:expr, $ty:ty) => {{
        let maybe_val = $value.parse::<$ty>();
        if maybe_val.is_err() { return None };
        maybe_val.unwrap()
    }};

    // call handler with params
    (@call_pure $request:expr, $handler:ident, $params:expr, $({$id:ident : $ty:ty : $idx:expr}),*) => {{
        $handler($request, $({
            let value = $params[$idx];
            router!(@parse_type value, $ty)
        }),*)
    }};

    // Extract params from route, 0 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+) => {{
        $handler($request)
    }};

    // Extract params from route, 1 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0})
    }};

    // Extract params from route, 2 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1})
    }};

    // Extract params from route, 3 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2})
    }};

    // Extract params from route, 4 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3})
    }};

    // Extract params from route, 5 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)* {$id5:ident : $ty5:ty} $($p5:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3}, {$id5 : $ty5 : 4})
    }};

    // Extract params from route, 6 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)* {$id5:ident : $ty5:ty} $($p5:ident)* {$id6:ident : $ty6:ty} $($p6:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3}, {$id5 : $ty5 : 4}, {$id6 : $ty6 : 5})
    }};

    // Extract params from route, 7 params case
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)* {$id5:ident : $ty5:ty} $($p5:ident)* {$id6:ident : $ty6:ty} $($p6:ident)* {$id7:ident : $ty7:ty} $($p7:ident)*) => {{
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3}, {$id5 : $ty5 : 4}, {$id6 : $ty6 : 5}, {$id6 : $ty6 : 6})
    }};

    // Test a particular route for match and forward to @call if there is match
    (@one_route_with_method $request:expr, $method:expr, $path:expr, $default:expr, $expected_method: expr, $handler:ident, $($path_segment:tt)*) => {{
        if $method != $expected_method { return None };
        let mut s = "^".to_string();
        $(
            s.push('/');
            let path_segment = stringify!($path_segment);
            if path_segment.starts_with('{') {
                s.push_str(r#"([\w-]+)"#);
            } else {
                s.push_str(path_segment);
            }
        )+
        s.push('$');
        let re = regex::Regex::new(&s).unwrap();
        if let Some(captures) = re.captures($path) {
            let matches: Vec<&str> = captures.iter().skip(1).filter(|x| x.is_some()).map(|x| x.unwrap().as_str()).collect();
            Some(router!(@call, $request, $handler, matches, $($path_segment)*))
        } else {
            None
        }
    }};

    // Transform GET token to Method::GET
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, GET, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::GET, $handler, $($path_segment)*)
    };

    // Transform POST token to Method::POST
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, POST, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::POST, $handler, $($path_segment)*)
    };
    // Transform PUT token to Method::PUT
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, PUT, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::PUT, $handler, $($path_segment)*)
    };
    // Transform PATCH token to Method::PATCH
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, PATCH, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::PATCH, $handler, $($path_segment)*)
    };
    // Transform DELETE token to Method::DELETE
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, DELETE, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::DELETE, $handler, $($path_segment)*)
    };
    // Transform OPTIONS token to Method::OPTIONS
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, OPTIONS, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::OPTIONS, $handler, $($path_segment)*)
    };

    // Transform HEAD token to Method::HEAD
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, HEAD, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::HEAD, $handler, $($path_segment)*)
    };

    // Transform TRACE token to Method::TRACE
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, TRACE, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::TRACE, $handler, $($path_segment)*)
    };

    // Transform CONNECT token to Method::CONNECT
    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, CONNECT, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::CONNECT, $handler, $($path_segment)*)
    };

    // Entry pattern
    ($($method_token:ident $(/$path_segment:tt)+ => $handler:ident),* , _ => $default:ident $(,)*) => {{
        |request, method: Method, path: &str| {
            let mut result = None;
            $(
                if result.is_none() {
                    // we use closure here so that we could make early return from macros inside of it
                    let closure = || {
                        router!(@one_route request, method, path, $default, $method_token, $handler, $($path_segment)*)
                    };
                    result = closure();
                }
            )*
            result.unwrap_or_else(|| $default(request))
        }
    }};

    (_ => $default:ident $(,)*) => {
        |request, _method: Method, _path: &str| {
            $default(request)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbs() {
        let get_test = |_: usize| Method::GET;
        let post_test = |_: usize| Method::POST;
        let put_test = |_: usize| Method::PUT;
        let patch_test = |_: usize| Method::PATCH;
        let delete_test = |_: usize| Method::DELETE;
        let connect_test = |_: usize| Method::CONNECT;
        let options_test = |_: usize| Method::OPTIONS;
        let trace_test = |_: usize| Method::TRACE;
        let head_test = |_: usize| Method::HEAD;
        let panic_test = |_: usize| unreachable!();
        let router = router!(
            GET /users => get_test,
            POST /users => post_test,
            PUT /users => put_test,
            PATCH /users => patch_test,
            DELETE /users => delete_test,
            OPTIONS /users => options_test,
            CONNECT /users => connect_test,
            TRACE /users => trace_test,
            HEAD /users => head_test,
            _ => panic_test
        );

        assert_eq!(router(0, Method::GET, "/users"), Method::GET);
        assert_eq!(router(0, Method::POST, "/users"), Method::POST);
        assert_eq!(router(0, Method::PUT, "/users"), Method::PUT);
        assert_eq!(router(0, Method::PATCH, "/users"), Method::PATCH);
        assert_eq!(router(0, Method::DELETE, "/users"), Method::DELETE);
        assert_eq!(router(0, Method::OPTIONS, "/users"), Method::OPTIONS);
        assert_eq!(router(0, Method::TRACE, "/users"), Method::TRACE);
        assert_eq!(router(0, Method::CONNECT, "/users"), Method::CONNECT);
        assert_eq!(router(0, Method::HEAD, "/users"), Method::HEAD);
    }

    fn yo(x: u32) -> u32 {
        println!("Called yo with {}", x);
        x
    }

    fn yo1(x: u32, y: String) -> u32 {
        println!("Called yo1 with {} and {}", x, y);
        x + 1
    }

    fn yo2(x: u32, y: String, z: u32) -> u32 {
        println!("Called yo2 with {} and {} and {}", x, y, z);
        x + 2
    }


    #[test]
    fn it_works() {
        // trace_macros!(true);
        // let router = router!(
        //     _ => yo,
        //     GET /users/{user_id}/accounts/{account_id}/transactions/{transaction_id} => yo
        // );
        let router = router!(
            // GET /users/transactions/{transaction_id: String}/accounts => yostr
            POST /users/transactions/{transaction_id: String}/accounts/{account_id: u32} => yo2,
            GET /users/transactions/{transaction_id: String}/accounts => yo1,
            _ => yo,
            // GET /users/transactions => yo
        );

        // trace_macros!(false);
        // router(32, Method::GET, "/users/transactions/trans_id_string/accounts/dgdfg");
        assert_eq!(router(32, Method::GET, "/users/transactions/trans_id_string/accounts"), 33);
        assert_eq!(router(32, Method::POST, "/users/transactions/trans_id_string/accounts/123"), 34);
        assert_eq!(router(32, Method::POST, "/users/transactions/trans_id_string/accounts/dgdfg"), 32);
        assert_eq!(router(32, Method::GET, "/users/transact"), 32);
    }
}


// cargo +nightly rustc -- -Zunstable-options --pretty=expanded


//     fn yo(x: u32) -> u32 {
//         println!("Called yo with {}", x);
//         x
//     }

//     fn yo1(x: u32, y: String) -> u32 {
//         println!("Called yo1 with {} and {}", x, y);
//         x + 1
//     }

//     fn yo2(x: u32, y: String, z: u32) -> u32 {
//         println!("Called yo2 with {} and {} and {}", x, y, z);
//         x + 2
//     }


// fn dgf132() {
//         let router = router!(
//             _ => yo,
//             // GET /users/transactions/{transaction_id: String}/accounts => yostr
//             GET /users/transactions/{transaction_id: String}/accounts/{account_id: u32} => yo2,
//             GET /users/transactions/{transaction_id: String}/accounts => yo1
//             // GET /users/transactions => yo
//         );    
// }