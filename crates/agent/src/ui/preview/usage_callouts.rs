use client::neopilot_urls;
use component::{empty_example, example_group_with_title, single_example};
use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use language_model::RequestUsage;
use neopilot_llm_client::{Plan, UsageLimit};
use ui::{Callout, Color, Icon, IconName, IconSize, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct UsageCallout {
    plan: Plan,
    usage: RequestUsage,
}

impl UsageCallout {
    pub fn new(plan: Plan, usage: RequestUsage) -> Self {
        Self { plan, usage }
    }
}

impl RenderOnce for UsageCallout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (is_limit_reached, is_approaching_limit, remaining) = match self.usage.limit {
            UsageLimit::Limited(limit) => {
                let percentage = self.usage.amount as f32 / limit as f32;
                let is_limit_reached = percentage >= 1.0;
                let is_near_limit = percentage >= 0.9 && percentage < 1.0;
                (
                    is_limit_reached,
                    is_near_limit,
                    limit.saturating_sub(self.usage.amount),
                )
            }
            UsageLimit::Unlimited => (false, false, 0),
        };

        if !is_limit_reached && !is_approaching_limit {
            return div().into_any_element();
        }

        let (title, message, button_text, url) = if is_limit_reached {
            match self.plan {
                Plan::NeopilotFree => (
                    "Out of free prompts",
                    "Upgrade to continue, wait for the next reset, or switch to API key."
                        .to_string(),
                    "Upgrade",
                    neopilot_urls::account_url(cx),
                ),
                Plan::NeopilotProTrial => (
                    "Out of trial prompts",
                    "Upgrade to Neopilot Pro to continue, or switch to API key.".to_string(),
                    "Upgrade",
                    neopilot_urls::account_url(cx),
                ),
                Plan::NeopilotPro => (
                    "Out of included prompts",
                    "Enable usage-based billing to continue.".to_string(),
                    "Manage",
                    neopilot_urls::account_url(cx),
                ),
            }
        } else {
            match self.plan {
                Plan::NeopilotFree => (
                    "Reaching free plan limit soon",
                    format!(
                        "{remaining} remaining - Upgrade to increase limit, or switch providers",
                    ),
                    "Upgrade",
                    neopilot_urls::account_url(cx),
                ),
                Plan::NeopilotProTrial => (
                    "Reaching trial limit soon",
                    format!(
                        "{remaining} remaining - Upgrade to increase limit, or switch providers",
                    ),
                    "Upgrade",
                    neopilot_urls::account_url(cx),
                ),
                _ => return div().into_any_element(),
            }
        };

        let icon = if is_limit_reached {
            Icon::new(IconName::X)
                .color(Color::Error)
                .size(IconSize::XSmall)
        } else {
            Icon::new(IconName::Warning)
                .color(Color::Warning)
                .size(IconSize::XSmall)
        };

        Callout::multi_line(
            title,
            message,
            icon,
            button_text,
            Box::new(move |_, _, cx| {
                cx.open_url(&url);
            }),
        )
        .into_any_element()
    }
}

impl Component for UsageCallout {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "AgentUsageCallout"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let free_examples = example_group_with_title(
            "Free Plan",
            vec![
                single_example(
                    "Approaching limit (90%)",
                    UsageCallout::new(
                        Plan::NeopilotFree,
                        RequestUsage {
                            limit: UsageLimit::Limited(50),
                            amount: 45, // 90% of limit
                        },
                    )
                    .into_any_element(),
                ),
                single_example(
                    "Limit reached (100%)",
                    UsageCallout::new(
                        Plan::NeopilotFree,
                        RequestUsage {
                            limit: UsageLimit::Limited(50),
                            amount: 50, // 100% of limit
                        },
                    )
                    .into_any_element(),
                ),
            ],
        );

        let trial_examples = example_group_with_title(
            "Neopilot Pro Trial",
            vec![
                single_example(
                    "Approaching limit (90%)",
                    UsageCallout::new(
                        Plan::NeopilotProTrial,
                        RequestUsage {
                            limit: UsageLimit::Limited(150),
                            amount: 135, // 90% of limit
                        },
                    )
                    .into_any_element(),
                ),
                single_example(
                    "Limit reached (100%)",
                    UsageCallout::new(
                        Plan::NeopilotProTrial,
                        RequestUsage {
                            limit: UsageLimit::Limited(150),
                            amount: 150, // 100% of limit
                        },
                    )
                    .into_any_element(),
                ),
            ],
        );

        let pro_examples = example_group_with_title(
            "Neopilot Pro",
            vec![
                single_example(
                    "Limit reached (100%)",
                    UsageCallout::new(
                        Plan::NeopilotPro,
                        RequestUsage {
                            limit: UsageLimit::Limited(500),
                            amount: 500, // 100% of limit
                        },
                    )
                    .into_any_element(),
                ),
                empty_example("Unlimited plan (no callout shown)"),
            ],
        );

        Some(
            div()
                .p_4()
                .flex()
                .flex_col()
                .gap_4()
                .child(free_examples)
                .child(trial_examples)
                .child(pro_examples)
                .into_any_element(),
        )
    }
}
