use crate::SqlxValues;
use sea_query::Value;

/// Bind `sea_query::Value`s into `sqlx_d1::D1Arguments`.
///
/// This mirrors the existing `sqlx-sqlite` binder where possible, but accounts for the fact that
/// `sqlx-d1` is intended for Cloudflare Workers (Wasm) and supports a smaller set of parameter
/// types.
impl<'q> sqlx::IntoArguments<'q, sqlx_d1::D1> for SqlxValues {
    fn into_arguments(self) -> <sqlx_d1::D1 as sqlx::Database>::Arguments<'q> {
        use sqlx::Arguments as _;

        let mut args: <sqlx_d1::D1 as sqlx::Database>::Arguments<'q> = Default::default();

        for value in self.0.into_iter() {
            // Keep this binder intentionally conservative and explicit: D1 is SQLite-like but not
            // identical, and not all type encodings available in `sqlx-sqlite` exist for `sqlx-d1`.
            match value {
                Value::Bool(v) => {
                    let _ = args.add(v);
                }
                Value::TinyInt(v) => {
                    let _ = args.add(v);
                }
                Value::SmallInt(v) => {
                    let _ = args.add(v);
                }
                Value::Int(v) => {
                    let _ = args.add(v);
                }
                Value::BigInt(v) => {
                    let _ = args.add(v);
                }
                Value::TinyUnsigned(v) => {
                    let _ = args.add(v);
                }
                Value::SmallUnsigned(v) => {
                    let _ = args.add(v);
                }
                Value::Unsigned(v) => {
                    let _ = args.add(v);
                }
                Value::BigUnsigned(v) => {
                    let _ = args.add(v);
                }
                Value::Float(v) => {
                    let _ = args.add(v);
                }
                Value::Double(v) => {
                    let _ = args.add(v);
                }
                Value::String(v) => {
                    let _ = args.add(v);
                }
                Value::Char(v) => {
                    let _ = args.add(v.map(|c| c.to_string()));
                }
                Value::Bytes(v) => {
                    let _ = args.add(v);
                }

                #[cfg(feature = "with-json")]
                Value::Json(v) => {
                    let _ = args.add(v.map(|j| sqlx::types::Json(*j)));
                }

                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(v) => {
                    let _ = args.add(v);
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(v) => {
                    let _ = args.add(v);
                }

                // `sqlx-d1` does not support the `time` crate types directly.
                // Encode them as TEXT to keep round-tripping deterministic.
                #[cfg(feature = "with-time")]
                Value::TimeDate(v) => {
                    let _ = args.add(v.map(|t| t.to_string()));
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(v) => {
                    let _ = args.add(v.map(|t| t.to_string()));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(v) => {
                    let _ = args.add(v.map(|t| t.to_string()));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(v) => {
                    let _ = args.add(v.map(|t| t.to_string()));
                }

                #[cfg(feature = "with-uuid")]
                Value::Uuid(v) => {
                    let _ = args.add(v);
                }

                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(v) => {
                    let _ = args.add(v);
                }

                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(v) => {
                    // Store BigDecimal as TEXT to preserve precision.
                    let _ = args.add(v.map(|d| d.to_string()));
                }

                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("D1 doesn't support SQL array arguments");
                }

                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    panic!("D1 doesn't support vector arguments");
                }

                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("D1 doesn't support IpNetwork arguments");
                }

                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("D1 doesn't support MacAddress arguments");
                }
            }
        }

        args
    }
}
