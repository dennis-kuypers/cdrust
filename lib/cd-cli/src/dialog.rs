use crate::prelude::*;

#[derive(StructOpt, Debug)]
pub struct DialogOpts {
    #[structopt(long)]
    /// Auto-respond confirmations with 'yes'
    pub yes: bool,

    #[structopt(long, conflicts_with = "any")]
    /// Automatically select all possible items
    pub all: bool,

    #[structopt(long)]
    /// Automatically pick one
    pub any: bool,

    #[structopt(short = "1", long)]
    /// If asked to select many, show dialog to select just one
    pub just_one: bool,
}

pub trait DialogProvider {
    fn dialog(&self, opts: DialogOpts) -> anyhow::Result<InteractiveOutput>;
}

impl DialogProvider for App {
    fn dialog(&self, opts: DialogOpts) -> anyhow::Result<InteractiveOutput> {
        ctrlc::set_handler(move || {
            let term = dialoguer::console::Term::stderr();
            let _ = term.show_cursor();
        })?;

        // let config = self.config("user")?;
        let select = match (opts.any, opts.all) {
            (true, false) => AutoSelect::First,
            (false, true) => AutoSelect::All,
            _ => AutoSelect::None,
        };
        Ok(InteractiveOutput::new(select, opts.just_one, opts.yes))
    }
}

#[derive(derive_new::new)]
pub struct InteractiveOutput {
    select: AutoSelect,
    just_one: bool,
    yes: bool,
}

#[derive(PartialEq, Debug)]
pub enum AutoSelect {
    None,
    All,
    First,
}

impl InteractiveOutput {
    pub fn confirm(&self, s: impl AsRef<str>) -> Option<bool> {
        if self.yes {
            info!("Auto-confirming dialog due to `-yes`");
            return Some(true);
        }

        dialoguer::Confirm::new().with_prompt(s.as_ref()).interact().ok()
    }

    pub fn select_one(&self, items: &[&str], s: impl AsRef<str>) -> Option<usize> {
        if !items.is_empty() && self.select == AutoSelect::First {
            info!("Auto-selecting first item");
            return Some(0);
        }

        let theme: Box<dyn dialoguer::theme::Theme> = if true {
            Box::new(dialoguer::theme::ColorfulTheme::default())
        } else {
            Box::new(dialoguer::theme::SimpleTheme)
        };

        dialoguer::FuzzySelect::with_theme(theme.as_ref())
            .items(items)
            .with_prompt(s.as_ref())
            .interact_opt()
            .ok()
            .flatten()
    }

    pub fn select(&self, items: &[&str], s: impl AsRef<str>, default: bool) -> Option<Vec<usize>> {
        if self.select == AutoSelect::First {
            info!("Auto-selecting first item");
            return Some(vec![0]);
        }

        if self.just_one {
            debug!("Showing single item selection for requested multi-selection");
            return self.select_one(items, s).map(|idx| vec![idx]);
        }

        if self.select == AutoSelect::All {
            info!("Auto-selecting all items");
            return Some((0..items.len()).collect());
        }

        let items_with_default: Vec<_> = items.iter().map(|&i| (i, default)).collect();

        let theme: Box<dyn dialoguer::theme::Theme> = if true {
            Box::new(dialoguer::theme::ColorfulTheme::default())
        } else {
            Box::new(dialoguer::theme::SimpleTheme)
        };

        dialoguer::MultiSelect::with_theme(theme.as_ref())
            .items_checked(items_with_default.as_slice())
            .with_prompt(s.as_ref())
            .interact_opt()
            .ok()
            .flatten()
    }
}
