use crate::cli::{Clients, Options, SubCommand, TimeEntries};
use crate::config::init_settings_file;
use clap::Parser;
use cli::Reports;
use client::init_client;
use report_client::init_report_client;

mod cli;
mod client;
mod commands;
mod config;
mod model;
mod report_client;

#[cfg(test)]
mod client_tests;

fn main() -> anyhow::Result<()> {
  let options = Options::parse();
  let format = options.format;
  let debug = options.debug;

  match options.subcommand {
    SubCommand::Init => init_settings_file()?,
    SubCommand::Projects(_action) => {
      let client = init_client()?;

      commands::projects::list(debug, &format, &client)?;
    }
    SubCommand::Workspaces(_action) => {
      let client = init_client()?;

      commands::workspaces::list(debug, &format, &client)?;
    }

    SubCommand::TimeEntries(action) => match action {
      TimeEntries::Create(time_entry) => {
        let client = init_client()?;
        commands::time_entries::create(debug, &format, &time_entry, &client)?
      }
      TimeEntries::List(list_time_entries) => {
        let client = init_client()?;
        commands::time_entries::list(
          debug,
          &format,
          &list_time_entries.range,
          list_time_entries.missing,
          &client,
        )?
      }
      TimeEntries::Start(time_entry) => {
        let client = init_client()?;
        commands::time_entries::start(debug, &format, &time_entry, &client)?
      }
      TimeEntries::Stop(time_entry) => {
        let client = init_client()?;
        commands::time_entries::stop(debug, &format, &time_entry, &client)?
      }
      TimeEntries::Delete(time_entry) => {
        let client = init_client()?;
        commands::time_entries::delete(debug, &format, &time_entry, &client)?
      }
    },

    SubCommand::Clients(action) => match action {
      Clients::Create(create_client) => {
        let client = init_client()?;
        commands::clients::create(debug, &format, &create_client, &client)?
      }
      Clients::List => {
        let client = init_client()?;
        commands::clients::list(debug, &format, &client)?;
      }
    },

    SubCommand::Reports(action) => match action {
      Reports::Detailed(detailed) => {
        let client = init_client()?;
        let report_client = init_report_client()?;

        commands::reports::detailed(
          debug,
          &client,
          &detailed.range,
          &report_client,
        )?;
      }
      Reports::Summary(detailed) => {
        let client = init_client()?;
        let report_client = init_report_client()?;

        commands::reports::summary(
          debug,
          &client,
          &detailed.range,
          &report_client,
        )?;
      }
    },
  }

  Ok(())
}
