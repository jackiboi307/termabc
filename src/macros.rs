/** A sort of dynamic version of formatf, that can be used with control sequence constants.
  * ```
  * formatf!(
  *     "{FG_RGB}{BG_RGB}Yellow text against a red background{RESET}\n",
  *     255, 255, 0,
  *     255, 0, 0
  * );
  * ```
  */
#[macro_export]
macro_rules! formatf {
    ($fmt:expr $(, $arg:expr)*) => {{
        let mut result = format!($fmt);
        $(
            result = result.replacen("{}", &$arg.to_string(), 1);
        )*
        result
    }};
}

/** A sort of dynamic version of printf, that can be used with control sequence constants.
  * ```
  * printf!(
  *     "{FG_RGB}{BG_RGB}Yellow text against a red background{RESET}\n",
  *     255, 255, 0,
  *     255, 0, 0
  * );
  * ```
  */
#[macro_export]
macro_rules! printf {
    ($fmt:expr $(, $arg:expr)*) => {{
        print!("{}", formatf!($fmt $(, $arg)*));
    }};
}
