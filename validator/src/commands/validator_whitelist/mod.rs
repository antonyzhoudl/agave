use {
    crate::{
        admin_rpc_service,
        commands::{FromClapArgMatches, Result},
    },
    clap::{App, Arg, ArgMatches, SubCommand},
    std::path::Path,
};

const COMMAND: &str = "validator-whitelist";

#[derive(Debug, PartialEq)]
pub struct ValidatorWhitelistArgs {
    pub path: String,
}

impl FromClapArgMatches for ValidatorWhitelistArgs {
    fn from_clap_arg_match(matches: &ArgMatches) -> Result<Self> {
        Ok(ValidatorWhitelistArgs {
            path: matches
                .value_of("path")
                .expect("path is required")
                .to_string(),
        })
    }
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name(COMMAND)
        .about("Filter the whitelist of validator nodes that transactions are sent to")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .takes_value(true)
                .required(true)
                .help(
                    "Provide a path to a json file containing a list of trusted validator",
                ),
        )
        .after_help(
            "Note: the new validator whitelist only applies to the currently running validator instance",
        )
}

pub fn execute(matches: &ArgMatches, ledger_path: &Path) -> Result<()> {
    let validator_whitelist_args = ValidatorWhitelistArgs::from_clap_arg_match(matches)?;

    let admin_client = admin_rpc_service::connect(ledger_path);
    admin_rpc_service::runtime().block_on(async move {
        admin_client
            .await?
            .set_validator_whitelist(validator_whitelist_args.path)
            .await
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::commands::tests::{
            verify_args_struct_by_command, verify_args_struct_by_command_is_error,
        },
    };

    #[test]
    fn verify_args_struct_by_command_validator_whitelist_default() {
        verify_args_struct_by_command_is_error::<ValidatorWhitelistArgs>(command(), vec![COMMAND]);
    }

    #[test]
    fn verify_args_struct_by_command_validator_whitelist_path() {
        verify_args_struct_by_command(
            command(),
            vec![COMMAND, "test.json"],
            ValidatorWhitelistArgs {
                path: "test.json".to_string(),
            },
        );
    }
}
