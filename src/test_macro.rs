#[macro_export]
macro_rules! check {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with: {}", stringify!($e), e),
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($e:expr,$closure:tt) => {
        match $e {
            Ok(t) => {
                $closure(t)
            },
            Err(e) => panic!("{} failed with: {}", stringify!($e), e),
        }
    };
}

#[macro_export]
macro_rules! error {
    ($e:expr, $s:expr) => {
        match $e {
            Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
            Err(ref err) => assert!(
                err.raw_os_error() == Some($s),
                format!("`{}` did not have a code of `{}`", err, $s)
            ),
        }
    };
}

#[macro_export]
macro_rules! error_contains {
    ($e:expr, $s:expr) => {
        match $e {
            Ok(_) => panic!("Unexpected success. Should've been: {:?}", $s),
            Err(ref err) => {
                assert!(err.to_string().contains($s), format!("`{}` did not contain `{}`", err, $s))
            }
        }
    };
}