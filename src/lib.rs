mod quill {

    use sea_query::*;

    struct Connection {
        uri: String,
    }

    impl Connection {
        pub fn from_uri(s: String) -> Connection {
            Connection {
                uri: s.clone(),
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use crate::quill;

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }

        #[test]
        fn create_connection() {
            let mut conn: quill::Connection = quill::Connection::from_uri(String::from("someuri"));
            assert_eq!(conn.uri, "someuri");
        }

        fn define_schema() {

            let schema = quill::Schema::new({
                quill::Model::new("users", {
                    quill::Column {
                        name: "id",
                        type: quill::Column::Type::PrimaryKey,
                        tokens: Vec<quill::Column::Token> {
                            quill::Column::Token::Unique(true),
                            quill::Column::Token::AutoIncrement(true),
                        },
                    },
                    quill::Column {
                        name: "username",
                        type: quill::Column::Type::Varchar(16),
                    },
                },
            });

            let query = quill::Query::from_db("database1")
            .select(&UserModel)
            .where(UserModel::Props::Username)
            .eq("axtyax");

            // execute sql query
            let results = sql_executor(query.to_string());

        }

    }

}
