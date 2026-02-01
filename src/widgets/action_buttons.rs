use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, row};
use cosmic::Element;
use cosmic_notifications_util::{ActionId, NotificationAction};

/// Maximum number of action buttons to display
const MAX_VISIBLE_ACTIONS: usize = 3;

/// Message type for action button clicks
#[derive(Debug, Clone)]
pub enum ActionMessage {
  Clicked(u32, String), // (notification_id, action_id)
}

/// Create a row of action buttons for a notification
pub fn action_buttons_row<'a, Message: Clone + 'a>(
  notification_id: u32,
  actions: &[NotificationAction],
  on_action: impl Fn(u32, String) -> Message + 'a + Clone,
) -> Element<'a, Message> {
  // Filter out default action and limit to MAX_VISIBLE_ACTIONS
  let visible_actions: Vec<_> = actions
    .iter()
    .filter(|a| a.id != "default")
    .take(MAX_VISIBLE_ACTIONS)
    .collect();

  if visible_actions.is_empty() {
    return cosmic::widget::Space::new(0, 0).into();
  }

  let buttons: Vec<Element<'a, Message>> = visible_actions
    .iter()
    .map(|action| {
      let action_id = action.id.clone();
      let on_action = on_action.clone();

      button::text(&action.label)
        .on_press((on_action)(notification_id, action_id))
        .padding([6, 12])
        .into()
    })
    .collect();

  row(buttons)
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

/// Create a single action button
pub fn action_button<'a, Message: Clone + 'a>(
  action: &NotificationAction,
  notification_id: u32,
  on_action: impl Fn(u32, String) -> Message + 'a,
) -> Element<'a, Message> {
  let action_id = action.id.clone();

  button::text(&action.label)
    .on_press(on_action(notification_id, action_id))
    .padding([6, 12])
    .into()
}

/// Check if there are any displayable actions (excluding default)
pub fn has_displayable_actions(actions: &[NotificationAction]) -> bool {
  actions.iter().any(|a| a.id != "default")
}

/// Convert from Notification tuple format to NotificationAction
pub fn convert_action_tuple(action: &(ActionId, String)) -> NotificationAction {
  NotificationAction {
    id: action.0.to_string(),
    label: action.1.clone(),
  }
}

/// Convert a slice of action tuples to NotificationActions
pub fn convert_actions(actions: &[(ActionId, String)]) -> Vec<NotificationAction> {
  actions.iter().map(convert_action_tuple).collect()
}
