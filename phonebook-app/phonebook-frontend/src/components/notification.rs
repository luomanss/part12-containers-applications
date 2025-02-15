use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct Notification {
    notification_type: NotificationType,
    message: String,
}

impl Notification {
    pub fn success(message: &str) -> Self {
        Self {
            notification_type: NotificationType::Success,
            message: message.to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            notification_type: NotificationType::Error,
            message: message.to_string(),
        }
    }
}

#[component]
pub fn notification(notification: ReadSignal<Option<Notification>>) -> impl IntoView {
    let success = move || {
        notification
            .get()
            .map(|n| n.notification_type == NotificationType::Success)
            .unwrap_or_default()
    };

    let error = move || {
        notification
            .get()
            .map(|n| n.notification_type == NotificationType::Error)
            .unwrap_or_default()
    };

    let message = move || notification.get().map(|n| n.message).unwrap_or_default();

    view! {
        <Show when=move || { notification.get().is_some() } >
            <div class="notification" class:success=success class:error=error>{message()}</div>
        </Show>
    }
}
