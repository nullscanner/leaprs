use leap_sys::LEAP_POLICY_EVENT;

use crate::PolicyFlags;

crate::leap_ref_struct!(
    #[doc = " The response from a request to get or set a policy."]
    #[doc = " LeapPollConnection() creates this struct when the response becomes available."]
    #[doc = " @since 3.0.0"]
    PolicyEvent,
    LEAP_POLICY_EVENT
);

impl<'a> PolicyEvent<'a> {
    #[doc = " A bitfield containing the policies effective at the"]
    #[doc = " time the policy event was processed. @since 3.0.0"]
    pub fn current_policy(&self) -> PolicyFlags {
        PolicyFlags::from_bits_truncate(self.handle.current_policy.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;
    #[test]
    fn policy_event() {
        let mut connection = initialize_connection();
        connection
            .set_policy_flags(PolicyFlags::IMAGES, PolicyFlags::empty())
            .expect("Failed to set policy");
        connection
            .wait_for(|e| match e {
                Event::Policy(e) => {
                    if e.current_policy().contains(PolicyFlags::IMAGES) {
                        Some(())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .expect("Did not receive an image policy event");
    }
}
