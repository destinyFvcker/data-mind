use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AuthError {}

#[cfg(test)]
mod test {
    use common_macro::stack_trace_debug;
    use serde::{Deserialize, Serialize};
    use snafu::{Location, ResultExt, Snafu};

    #[derive(Snafu)]
    #[stack_trace_debug]
    enum SimpleError {
        #[snafu(display("Failed to deserialize value"))]
        ValueDeserialize {
            #[snafu(source)]
            error: serde_json::error::Error, // <-- external source
            #[snafu(implicit)]
            location: Location,
        },

        #[allow(unused)]
        #[snafu(display("Table engine not found: {}", engine_name))]
        TableEngineNotFound {
            engine_name: String,
            #[snafu(implicit)]
            location: Location,
            source: common_error::mock::MockError, // <-- internal source
        },
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct SimpleStruct {
        filed: String,
    }

    async fn decode_msg(msg: &[u8]) -> Result<SimpleStruct, SimpleError> {
        serde_json::from_slice(&msg).context(ValueDeserializeSnafu) // propagate error with new stack and context
    }

    #[actix_web::test]
    async fn test_common_macro() {
        let simple1 = SimpleStruct {
            filed: "simple struct 1".to_string(),
        };

        let simple1_json = serde_json::to_string(&simple1).unwrap();
        let simple1_bytes = simple1_json.bytes().collect::<Vec<u8>>();

        let result1 = decode_msg(&simple1_bytes).await;

        match result1 {
            Ok(struc) => {
                println!("{:?}", struc);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        let result2 = decode_msg(&simple1_bytes[1..]).await;
        match result2 {
            Ok(struc) => {
                println!("{:?}", struc);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
