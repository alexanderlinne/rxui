pub trait Observer<Observation, Item, Error> {
    fn on_subscribe(&mut self, subscription: Observation);
    fn on_next(&mut self, item: Item);
    fn on_error(&mut self, errpr: Error);
    fn on_completed(&mut self);
}

impl<'o, Observation, Item, Error> Observer<Observation, Item, Error>
    for Box<dyn Observer<Observation, Item, Error> + 'o>
{
    fn on_subscribe(&mut self, subscription: Observation) {
        (&mut **self).on_subscribe(subscription)
    }

    fn on_next(&mut self, item: Item) {
        (&mut **self).on_next(item)
    }

    fn on_error(&mut self, error: Error) {
        (&mut **self).on_error(error)
    }

    fn on_completed(&mut self) {
        (&mut **self).on_completed()
    }
}

impl<Observation, Item, Error> Observer<Observation, Item, Error>
    for Box<dyn Observer<Observation, Item, Error> + Send + Sync + 'static>
{
    fn on_subscribe(&mut self, subscription: Observation) {
        (&mut **self).on_subscribe(subscription)
    }

    fn on_next(&mut self, item: Item) {
        (&mut **self).on_next(item)
    }

    fn on_error(&mut self, error: Error) {
        (&mut **self).on_error(error)
    }

    fn on_completed(&mut self) {
        (&mut **self).on_completed()
    }
}
