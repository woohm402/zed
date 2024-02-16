#[cfg(test)]
mod fake;
mod open_ai;
mod zed_dot_dev;

use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AppContext, AsyncWindowContext, Task};
use open_ai::*;
use settings::Settings;
use util::ResultExt;
use zed_dot_dev::*;

use crate::{assistant_settings::AssistantSettings, LanguageModelRequest};

#[derive(Clone)]
pub enum CompletionProvider {
    OpenAi(OpenAiCompletionProvider),
    ZedDotDev(ZedDotDevCompletionProvider),
    #[cfg(test)]
    Fake(fake::FakeCompletionProvider),
}

impl gpui::Global for CompletionProvider {}

impl CompletionProvider {
    #[cfg(test)]
    pub fn fake() -> Self {
        Self::Fake(fake::FakeCompletionProvider::default())
    }

    pub fn global(cx: &mut AppContext) -> Self {
        if !cx.has_global::<Self>() {}

        cx.global::<Self>().clone()
    }

    pub fn is_authenticated(&self) -> bool {
        match self {
            CompletionProvider::OpenAi(provider) => provider.is_authenticated(),
            CompletionProvider::ZedDotDev(provider) => provider.is_authenticated(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => true,
        }
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authenticate(cx),
            CompletionProvider::ZedDotDev(provider) => provider.authenticate(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.complete(request),
            CompletionProvider::ZedDotDev(_) => todo!(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => todo!(),
        }
    }

    #[cfg(test)]
    pub fn as_fake(&self) -> &fake::FakeCompletionProvider {
        match self {
            CompletionProvider::Fake(provider) => provider,
            _ => panic!(),
        }
    }
}
