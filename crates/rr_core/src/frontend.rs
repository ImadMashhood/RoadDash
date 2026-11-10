//! Entry navigation. Online transport and simulation remain outside this pure flow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Splash,
    Main,
    Online,
    Host,
    Join,
    Lobby,
    Rider,
    Street,
    Courses,
    Garage,
    Bar,
    Portrait,
    Starting,
    Results,
    Quit,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Thrash,
    Street,
}
/// Offline course identity, independent of rendering and world lifetime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Course {
    Metropolis,
    Hills,
    Valley,
    City,
}
impl Course {
    pub const ALL: [Self; 4] = [Self::Metropolis, Self::Hills, Self::Valley, Self::City];
    pub fn key(self) -> &'static str {
        match self {
            Self::Metropolis => "metro",
            Self::Hills => "hills",
            Self::Valley => "valley",
            Self::City => "city",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Metropolis => "METROPOLIS",
            Self::Hills => "HILLS",
            Self::Valley => "VALLEY RUN",
            Self::City => "CITY RUN",
        }
    }
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.key() == key)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Continue,
    Thrash,
    Online,
    Host,
    Join,
    QuickPlay,
    TogglePublic,
    NextMap,
    CreateLobby,
    JoinLobby,
    EditCode,
    ToggleCodeType,
    CodePrevious,
    CodeNext,
    CodeUp,
    CodeDown,
    StartOnline,
    Street,
    Courses,
    Garage,
    Bar,
    Back,
    Ride,
    Settings,
    Quit,
    ConfirmQuit,
    PreviousPortrait,
    NextPortrait,
    SelectPortrait(usize),
    SelectCourse(Course),
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Effect {
    #[default]
    None,
    Settings,
    Start,
    Exit,
}
#[derive(Clone, Debug, Default)]
pub struct Flow {
    pub page: Page,
    pub mode: Mode,
    pub portrait: usize,
    pub course: Option<Course>,
}
impl Flow {
    pub fn dispatch(&mut self, action: Action) -> Effect {
        use Action as A;
        use Page as P;
        let next = match (self.page, action) {
            (P::Courses, A::SelectCourse(course)) => {
                self.course = Some(course);
                return Effect::None;
            }
            (P::Results, A::Courses) => P::Courses,
            (P::Portrait, A::PreviousPortrait) => {
                self.portrait = (self.portrait + ROSTER.len() - 1) % ROSTER.len();
                return Effect::None;
            }
            (P::Portrait, A::NextPortrait) => {
                self.portrait = (self.portrait + 1) % ROSTER.len();
                return Effect::None;
            }
            (P::Bar, A::SelectPortrait(index)) if index < ROSTER.len() => {
                self.portrait = index;
                P::Portrait
            }
            (P::Portrait, A::Back) => P::Bar,
            (P::Splash, A::Continue) => P::Main,
            (P::Main, A::Online) => P::Online,
            (P::Online, A::Host) => P::Host,
            (P::Online, A::Join) => P::Join,
            (P::Host | P::Join | P::Lobby, A::Back) => P::Online,
            (P::Online, A::Back) => P::Main,
            (P::Main, A::Thrash) => {
                self.mode = Mode::Thrash;
                P::Courses
            }
            (P::Main, A::Street) => {
                self.mode = Mode::Street;
                P::Rider
            }
            (P::Rider, A::Continue) => P::Street,
            (P::Street, A::Courses) => P::Courses,
            (P::Street, A::Garage) => P::Garage,
            (P::Street, A::Bar) => P::Bar,
            (P::Courses | P::Results, A::Ride) => {
                self.page = P::Starting;
                return Effect::Start;
            }
            (P::Main | P::Street, A::Settings) => return Effect::Settings,
            (P::Main, A::Quit) => P::Quit,
            (P::Quit, A::ConfirmQuit) => return Effect::Exit,
            (P::Main, A::Back) => P::Quit,
            (P::Garage | P::Bar, A::Back) => P::Street,
            (P::Courses, A::Back) if self.mode == Mode::Street => P::Street,
            (P::Rider | P::Street | P::Courses | P::Results | P::Quit, A::Back) => P::Main,
            _ => return Effect::None,
        };
        self.page = next;
        Effect::None
    }
}

/// Approved concept portraits; independent of the current simulation roster.
pub struct Portrait {
    pub id: &'static str,
    pub name: &'static str,
    pub bio: &'static str,
}
pub const ROSTER: [Portrait; 16] = [
    Portrait {
        id: "yamakha",
        name: "Yamakha",
        bio: "Yamakha would rather watch a fight than join it. Beneath that ridiculous nail-studded cap, he is counting every mistake. He hangs back while the loudmouths batter each other, then slips past the wreckage. Stay on your feet: his idea of an opening includes a rider lying in the road.",
    },
    Portrait {
        id: "tyrone",
        name: "Tyrone White",
        bio: "Bald head, black jacket, enormous grin. Tyrone White is a big man who has spent good money looking exactly this pleased with himself. His real investment is in his legs: one well-timed kick can make an overtake somebody else's problem. Do not mistake the smile for an invitation to ride alongside.",
    },
    Portrait {
        id: "esteban",
        name: "Esteban Gonzalez",
        bio: "Esteban Gonzalez works construction, rides hard, and treats every break like an eating contest. His enormous sombrero and sauce-covered work vest are difficult to miss. Tacos, enchiladas, another taco: somehow there is always room for more. He is a hard worker with very little respect for a clean shirt.",
    },
    Portrait {
        id: "osama",
        name: "Osama bin Countryistani",
        bio: "Wrapped in a flowing scarf and spotless riding clothes, Osama bin Countryistani acts as if the entire road belongs to him. He is aggressive, theatrical, and far too fond of that whip. He would rather make an elaborate scene than accept a quiet defeat. Passing him is easy. Getting him to accept it is the problem.",
    },
    Portrait {
        id: "naveen",
        name: "Naveen Gowda Nambiar",
        bio: "Naveen Gowda Nambiar came to race, not trade punches. He studies the route, ducks around the fighting, and would much rather arrive first with his lunch still intact. Unfortunately, a great deal of that lunch is already on his sleeve. His navigation deserves more confidence than the upside-down map suggests.",
    },
    Portrait {
        id: "ellen",
        name: "Ellen Windyke",
        bio: "Blue hair, battered leather, and absolutely no patience for being talked down to. Ellen Windyke loves women, hates a patronizing lecture, and thinks the patriarchy could use a good kicking. She is not here to be anybody's decoration. Start explaining how she ought to ride and see how long your explanation lasts.",
    },
    Portrait {
        id: "luigi",
        name: "Luigi’s Pizzeria",
        bio: "Luigi named himself after his own pizzeria because race sponsorship costs money and shouting is free. He is skinny, slick-haired, and permanently wearing an apron with half the menu on it. That rolling pin is for dough, allegedly. He would like you to remember the restaurant even if you forget the beating.",
    },
    Portrait {
        id: "chinese-racer",
        name: "Pee Pi Long",
        bio: "Pee Pi Long does not waste much time talking. The hat is wide, the stare is sharp, and the punches arrive in an unpleasant hurry. He is an aggressive close-range fighter who wants to get alongside and stay there. If you are still deciding whether to swing, he has probably already swung twice.",
    },
    Portrait {
        id: "hog",
        name: "Hog",
        bio: "Hog is a 1950s greaser who appears to have taken a wrong turn through time. White tee tucked into jeans, enormous slicked-back hair, and enough swagger to fill a garage. Modern motorcycles leave him wide-eyed. He may not understand the electronics, but he understands looking cool while pretending nothing surprises him.",
    },
    Portrait {
        id: "cleetus",
        name: "Cleetus Mayballs",
        bio: "Cleetus Mayballs is a loud, sun-baked Florida menace with a short temper and a very long list of people he blames for everything. He can start an argument with a mirror and still insist the other fellow began it. Expect shouting, dirty riding, and an elaborate excuse whenever he loses.",
    },
    Portrait {
        id: "abeyomi",
        name: "Abeyomi",
        bio: "Abeyomi is a sharply dressed Nigerian professional with his route planned and his equipment in order. He is here to win, not put on a roadside circus. The bat is insurance against riders who cannot leave a clean racer alone. Give him a fair race and he will give you a very difficult one.",
    },
    Portrait {
        id: "shaqueesha",
        name: "Sha’queesha",
        bio: "Sha'queesha has six kids, a bonnet that refuses to budge, and no interest in going home without the prize money. She is wiry, determined, and already short on patience before the flag drops. Racing is expensive. Raising a family is worse. Anyone blocking the winnings is going to hear about it.",
    },
    Portrait {
        id: "anastasia",
        name: "Anastasia Jakmiova",
        bio: "Anastasia Jakmiova brings a dancer's balance and timing to a sport with considerably less applause. Angular, poised, and slippery as a spy in a bad movie, she would rather glide past a brawl than join it. She makes an overtake look rehearsed. The look she gives you afterward certainly was.",
    },
    Portrait {
        id: "george",
        name: "George Suckalopolis",
        bio: "George Suckalopolis is a Greek chef with flour on his apron and a talent for looking guilty before anything has happened. He argues with Elif about practically everything, particularly whose cooking deserves the credit. Between the kitchen and the starting line, he has developed a remarkable ability to make a meal of a simple problem.",
    },
    Portrait {
        id: "elif",
        name: "Elif Suckalopolkizi",
        bio: "Elif Suckalopolkizi is a Turkish racer with a cigarette, a helmet, and a deeply unimpressed opinion of the room. George's endless arguments are background noise by now. She takes her time answering, lets the smoke do the punctuation, and would prefer everyone else to stop talking long enough for her to ride.",
    },
    Portrait {
        id: "dolph",
        name: "Dolph Hiller",
        bio: "Dolph Hiller dresses as if the race ends at a formal banquet. The immaculate suit, tiny moustache, and absurd self-importance cannot conceal a cross-eyed, drooling fool with a violent temper. He expects obedience, admiration, and probably a medal. What he usually gets is somebody laughing at him from the next lane.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn portrait_navigation_wraps_only_inside_the_bar() {
        let mut flow = Flow::default();
        flow.dispatch(Action::NextPortrait);
        assert_eq!(flow.portrait, 0);
        flow.page = Page::Portrait;
        flow.dispatch(Action::PreviousPortrait);
        assert_eq!(flow.portrait, 15);
        for _ in 0..16 {
            assert_eq!(flow.dispatch(Action::NextPortrait), Effect::None);
        }
        assert_eq!(flow.portrait, 15);
        assert_eq!(flow.page, Page::Portrait);
        flow.dispatch(Action::Back);
        assert_eq!(flow.page, Page::Bar);
        flow.dispatch(Action::NextPortrait);
        assert_eq!(flow.portrait, 15);
        flow.dispatch(Action::SelectPortrait(3));
        assert_eq!(flow.page, Page::Portrait);
        assert_eq!(flow.portrait, 3);
        flow.dispatch(Action::Back);
        flow.dispatch(Action::SelectPortrait(16));
        assert_eq!(flow.page, Page::Bar);
        let ids: std::collections::HashSet<_> = ROSTER.iter().map(|r| r.id).collect();
        assert_eq!(ids.len(), 16);
    }
    #[test]
    fn thrash_and_street_paths_only_launch_from_a_course_or_results() {
        for street in [false, true] {
            let mut f = Flow::default();
            assert_eq!(f.dispatch(Action::Ride), Effect::None);
            f.dispatch(Action::Continue);
            f.dispatch(if street {
                Action::Street
            } else {
                Action::Thrash
            });
            if street {
                assert_eq!(f.page, Page::Rider);
                f.dispatch(Action::Continue);
                for a in [Action::Garage, Action::Bar] {
                    f.dispatch(a);
                    assert_eq!(f.dispatch(Action::Ride), Effect::None);
                    f.dispatch(Action::Back);
                    assert_eq!(f.page, Page::Street);
                }
                f.dispatch(Action::Courses);
            }
            assert_eq!(f.page, Page::Courses);
            assert_eq!(f.dispatch(Action::Ride), Effect::Start);
            assert_eq!(f.dispatch(Action::Ride), Effect::None);
            assert_eq!(f.dispatch(Action::Back), Effect::None);
        }
    }
    #[test]
    fn selecting_a_course_does_not_launch_and_results_can_return_to_selection() {
        let mut f = Flow {
            page: Page::Courses,
            ..Default::default()
        };
        for c in Course::ALL {
            assert_eq!(f.dispatch(Action::SelectCourse(c)), Effect::None);
            assert_eq!(f.page, Page::Courses);
            assert_eq!(f.course, Some(c));
            assert_eq!(Course::from_key(c.key()), Some(c));
        }
        assert_eq!(f.dispatch(Action::Ride), Effect::Start);
        f.page = Page::Results;
        assert_eq!(f.dispatch(Action::Courses), Effect::None);
        assert_eq!(f.page, Page::Courses);
        assert_eq!(f.course, Some(Course::City));
    }
    #[test]
    fn back_restores_the_correct_hub_and_settings_does_not_lose_it() {
        let mut f = Flow {
            page: Page::Street,
            mode: Mode::Street,
            ..Default::default()
        };
        assert_eq!(f.dispatch(Action::Settings), Effect::Settings);
        assert_eq!(f.page, Page::Street);
        f.dispatch(Action::Courses);
        f.dispatch(Action::Back);
        assert_eq!(f.page, Page::Street);
        f.dispatch(Action::Back);
        f.dispatch(Action::Back);
        assert_eq!(f.page, Page::Quit);
        f.dispatch(Action::Back);
        assert_eq!(f.page, Page::Main);
        assert_eq!(f.dispatch(Action::ConfirmQuit), Effect::None);
    }
}
