use std::sync::Arc;

// 共有リソースのためのヘルパー関数
pub fn shared<T>(resource: T) -> Arc<T> {
    Arc::new(resource)
}

// 関数合成のためのヘルパーユーティリティ
pub struct Compose<F, A, B> {
    f: F,
    _phantom: std::marker::PhantomData<(A, B)>,
}

// 型制約のあるComposeのエイリアス
pub type BoxedCompose<A, B> = Compose<Box<dyn Fn(A) -> B>, A, B>;

impl<A, B, F> Compose<F, A, B>
where
    F: Fn(A) -> B,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn then<C, G>(self, g: G) -> BoxedCompose<A, C>
    where
        G: Fn(B) -> C + Clone + 'static,
        F: Fn(A) -> B + Clone + 'static,
        A: 'static,
        B: 'static,
        C: 'static,
    {
        let f_clone = self.f.clone();
        let g_clone = g.clone();

        BoxedCompose {
            f: Box::new(move |a| g_clone((f_clone)(a))),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn apply(&self, a: A) -> B {
        (self.f)(a)
    }
}

// 非同期関数のための関数合成ユーティリティ
pub struct AsyncCompose<F, A, B, Fut> {
    f: F,
    _phantom: std::marker::PhantomData<(A, B, Fut)>,
}

pub type BoxedAsyncCompose<A, B> = AsyncCompose<Box<dyn Fn(A) -> std::pin::Pin<Box<dyn std::future::Future<Output = B> + Send>> + Send + Sync>, A, B, std::pin::Pin<Box<dyn std::future::Future<Output = B> + Send>>>;

impl<A, B, Fut, F> AsyncCompose<F, A, B, Fut>
where
    F: Fn(A) -> Fut,
    Fut: std::future::Future<Output = B>,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn then<C, G, FutG>(self, g: G) -> BoxedAsyncCompose<A, C>
    where
        G: Fn(B) -> FutG + Send + Sync + Clone + 'static,
        FutG: std::future::Future<Output = C> + Send + 'static,
        F: Fn(A) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = B> + Send + 'static,
        A: Send + 'static,
        B: Send + 'static,
        C: Send + 'static,
    {
        let f_clone = self.f.clone();
        let g_clone = g.clone();

        BoxedAsyncCompose {
            f: Box::new(move |a| {
                let f2 = f_clone.clone();
                let g2 = g_clone.clone();
                Box::pin(async move {
                    let b = (f2)(a).await;
                    g2(b).await
                })
            }),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn apply(&self, a: A) -> B {
        (self.f)(a).await
    }
}

// 使いやすいファサード関数
pub fn compose<A, B, F>(f: F) -> Compose<F, A, B>
where
    F: Fn(A) -> B,
{
    Compose::new(f)
}

pub fn boxed_compose<A, B, F>(f: F) -> BoxedCompose<A, B>
where
    F: Fn(A) -> B + 'static,
    A: 'static,
    B: 'static,
{
    BoxedCompose {
        f: Box::new(f),
        _phantom: std::marker::PhantomData,
    }
}

// 非同期合成のためのファサード関数
pub fn async_compose<A, B, Fut, F>(f: F) -> AsyncCompose<F, A, B, Fut>
where
    F: Fn(A) -> Fut,
    Fut: std::future::Future<Output = B>,
{
    AsyncCompose::new(f)
}

pub fn boxed_async_compose<A, B, Fut, F>(f: F) -> BoxedAsyncCompose<A, B>
where
    F: Fn(A) -> Fut + Send + Sync + Clone + 'static,
    Fut: std::future::Future<Output = B> + Send + 'static,
    A: Send + 'static,
    B: Send + 'static,
{
    let f_clone = f.clone();

    BoxedAsyncCompose {
        f: Box::new(move |a| Box::pin(f_clone(a))),
        _phantom: std::marker::PhantomData,
    }
}
