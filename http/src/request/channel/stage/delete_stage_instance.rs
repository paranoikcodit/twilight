use crate::request::prelude::*;
use twilight_model::id::ChannelId;

/// Delete the stage instance of a stage channel.
///
/// Requires the user to be a moderator of the stage channel.
pub struct DeleteStageInstance<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> DeleteStageInstance<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteStageInstance {
                channel_id: self.channel_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteStageInstance<'_>, ());