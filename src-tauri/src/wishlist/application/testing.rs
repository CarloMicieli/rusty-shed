use crate::wishlist::domain::MockWishlistRepository;
use crate::wishlist::domain::repository::{WishlistRepository, WishlistUowExt};

#[derive(Default)]
pub struct FakeUow {
    repo: Option<MockWishlistRepository>,
}

impl FakeUow {
    pub fn new(repo: MockWishlistRepository) -> Self {
        Self { repo: Some(repo) }
    }
}

impl WishlistUowExt for FakeUow {
    fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
        Box::new(self.repo.take().expect("wishlist repo already taken"))
    }
}
