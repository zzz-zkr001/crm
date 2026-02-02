use prost_builder_trait::tonic::BuilderAttributes;

fn main() -> anyhow::Result<()> {
    std::fs::create_dir_all("src/pb")?;
    tonic_prost_build::configure()
        .out_dir("src/pb")
        .with_serde(
            &["User"],
            true,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_derive_builder(
            &[
                "User",
                "QueryResult",
                "RawQueryResult",
                "TimeQuery",
                "IdQuery",
            ],
            None,
        )
        .with_type_attributes(
            &["User.email", "User.name", "RawQueryRequest.query"],
            &["#[builder(setter(into))]"],
        )
        .with_field_attributes(
            &["TimeQuery.before", "TimeQuery.after"],
            &["#[builder(setter(into))]"],
        )
        .compile_protos(
            &[
                "../protos/user-stats/messages.proto",
                "../protos/user-stats/rpc.proto",
            ],
            &["../protos/user-stats"],
        )?;
    // let builder = tonic_prost_build::configure();
    // builder
    //     .out_dir("src/pb")
    //     .compile_protos(&["../protos/crm/crm.proto"], &["../protos"])?;
    Ok(())
}
