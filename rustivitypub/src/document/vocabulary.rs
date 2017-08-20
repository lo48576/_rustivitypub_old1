//! ActivityStreams and ActivityPub vocabulary.

use std::error;
use std::fmt;


/// An ActivityPub document type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, AsRefStr, ToString)]
pub enum DocumentType {
    /// \[Core type\] `Object`.
    ///
    /// > Describes an object of any kind.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-object).
    Object,
    /// \[Core type\] `Link`.
    ///
    /// > A Link is an indirect, qualified reference to a resource identified by a URL.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-link).
    Link,
    /// \[Core type\] `Activity`.
    ///
    /// > An Activity is a subtype of `Object` that describes some form of action that may happen,
    /// > is currently happening, or has already happened.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-activity).
    Activity,
    /// \[Core type\] `IntransitiveActivity`.
    ///
    /// > Instances of `IntransitiveActivity` are a subtype of `Activity` representing intransitive
    /// > actions.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-intransitiveactivity).
    IntransitiveActivity,
    /// \[Core type\] `Collection`.
    ///
    /// > A `Collection` is a subtype of `Object` that represents ordered or unordered sets of
    /// > `Object` or `Link` instances.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-collection).
    Collection,
    /// \[Core type\] `OrderedCollection`.
    ///
    /// > A subtype of `Collection` in which members of the logical collection are assumed to
    /// > always be strictly ordered.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-orderedcollection).
    OrderedCollection,
    /// \[Core type\] `CollectionPage`.
    ///
    /// > Used to represent distinct subsets of items from a `Collection`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-collectionpage).
    CollectionPage,
    /// \[Core type\] `OrderedCollectionPage`.
    ///
    /// > Used to represent ordered subsets of items from an `OrderedCollection`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 2. Core
    /// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-orderedcollectionpage).
    OrderedCollectionPage,
    /// \[Activity type\] `Accept`.
    ///
    /// > Indicates that the `actor` accepts the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept).
    Accept,
    /// \[Activity type\] `Add`.
    ///
    /// > Indicates that the `actor` has added the `object` to the `target`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add).
    Add,
    /// \[Activity type\] `Announce`.
    ///
    /// > Indicates that the `actor` is calling the target's attention the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce).
    Announce,
    /// \[Activity type\] `Arrive`.
    ///
    /// > An `IntransitiveActivity` that indicates that the `actor` has arrived at the `location`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive).
    Arrive,
    /// \[Activity type\] `Block`.
    ///
    /// > Indicates that the `actor` is blocking the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block).
    Block,
    /// \[Activity type\] `Create`.
    ///
    /// > Indicates that the `actor` has created the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create).
    Create,
    /// \[Activity type\] `Delete`.
    ///
    /// > Indicates that the `actor` has deleted the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete).
    Delete,
    /// \[Activity type\] `Dislike`.
    ///
    /// > Indicates that the `actor` dislikes the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike).
    Dislike,
    /// \[Activity type\] `Flag`.
    ///
    /// > Indicates that the `actor` is "flagging" the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag).
    Flag,
    /// \[Activity type\] `Follow`.
    ///
    /// > Indicates that the `actor` is "following" the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow).
    Follow,
    /// \[Activity type\] `Ignore`.
    ///
    /// > Indicates that the `actor` is ignoring the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore).
    Ignore,
    /// \[Activity type\] `Invite`.
    ///
    /// > A specialization of `Offer` in which the `actor` is extending an invitation for the
    /// > `object` to the target.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite).
    Invite,
    /// \[Activity type\] `Join`.
    ///
    /// > Indicates that the `actor` has joined the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join).
    Join,
    /// \[Activity type\] `Leave`.
    ///
    /// > Indicates that the `actor` has left the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave).
    Leave,
    /// \[Activity type\] `Like`.
    ///
    /// > Indicates that the `actor` likes, recommends or endorses the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like).
    Like,
    /// \[Activity type\] `Listen`.
    ///
    /// > Indicates that the `actor` has listened to the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen).
    Listen,
    /// \[Activity type\] `Move`.
    ///
    /// > Indicates that the `actor` has moved `object` from `origin` to `target`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move).
    Move,
    /// \[Activity type\] `Offer`.
    ///
    /// > Indicates that the `actor` is offering the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer).
    Offer,
    /// \[Activity type\] `Question`.
    ///
    /// > Represents a question being asked.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question).
    Question,
    /// \[Activity type\] `Reject`.
    ///
    /// > Indicates that the `actor` is rejecting the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject).
    Reject,
    /// \[Activity type\] `Read`.
    ///
    /// > Indicates that the `actor` has read the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read).
    Read,
    /// \[Activity type\] `Remove`.
    ///
    /// > Indicates that the `actor` is removing the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove).
    Remove,
    /// \[Activity type\] `TentativeAccept`.
    ///
    /// > A specialization of `Accept` indicating that the acceptance is tentative.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept).
    TentativeAccept,
    /// \[Activity type\] `TentativeReject`.
    ///
    /// > A specialization of `Reject` in which the rejection is considered tentative.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject).
    TentativeReject,
    /// \[Activity type\] `Travel`.
    ///
    /// > Indicates that the `actor` is traveling to `target` from `origin`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel).
    Travel,
    /// \[Activity type\] `Undo`.
    ///
    /// > Indicates that the `actor` is undoing the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo).
    Undo,
    /// \[Activity type\] `Update`.
    ///
    /// > Indicates that the `actor` has updated the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update).
    Update,
    /// \[Activity type\] `View`.
    ///
    /// > Indicates that the `actor` has viewed the `object`.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.1 Activity
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view).
    View,
    /// \[Actor type\] `Application`.
    ///
    /// > Describes a software application.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.2 Actor
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application).
    Application,
    /// \[Actor type\] `Group`.
    ///
    /// > Represents a formal or informal collective of Actors.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.2 Actor
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group).
    Group,
    /// \[Actor type\] `Organization`.
    ///
    /// > Represents an organization.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.2 Actor
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization).
    Organization,
    /// \[Actor type\] `Person`.
    ///
    /// > Represents an individual person.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.2 Actor
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person).
    Person,
    /// \[Actor type\] `Service`.
    ///
    /// > Represents a service of any kind.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.2 Actor
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service).
    Service,
    /// \[Object type\] `Article`.
    ///
    /// > Represents any kind of multi-paragraph written work.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-article).
    Article,
    /// \[Object type\] `Audio`.
    ///
    /// > Represents an audio document of any kind.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audio).
    Audio,
    /// \[Object type\] `Document`.
    ///
    /// > Represents a document of any kind.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document).
    Document,
    /// \[Object type\] `Event`.
    ///
    /// > Represents any kind of event.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-event).
    Event,
    /// \[Object type\] `Image`.
    ///
    /// > An image document of any kind
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-image).
    Image,
    /// \[Object type\] `Note`.
    ///
    /// > Represents a short written work typically less than a single paragraph in length.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note).
    Note,
    /// \[Object type\] `Page`.
    ///
    /// > Represents a Web Page.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page).
    Page,
    /// \[Object type\] `Place`.
    ///
    /// > Represents a logical or physical location.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-place).
    Place,
    /// \[Object type\] `Profile`.
    ///
    /// > A Profile is a content object that describes another Object, typically used to describe
    /// > [Actor Type](https://www.w3.org/TR/activitystreams-vocabulary/#actor-types) objects.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-profile).
    Profile,
    /// \[Object type\] `Relationship`.
    ///
    /// > Describes a relationship between two individuals.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship).
    Relationship,
    /// \[Object type\] `Tombstone`.
    ///
    /// > A Tombstone represents a content object that has been deleted.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tombstone).
    Tombstone,
    /// \[Object type\] `Video`.
    ///
    /// > Represents a video document of any kind.
    ///
    /// See [\[REC-activitystreams-vocabulary-20170523\] 3.3 Object and Link
    /// Types](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-video).
    Video,
}


/// An error which indicates unknown document type.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownDocumentTypeError;

impl fmt::Display for UnknownDocumentTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown ActivityPub document type")
    }
}

impl error::Error for UnknownDocumentTypeError {
    fn description(&self) -> &str {
        "Unknown ActivityPub document type"
    }
}
