use std::io;

use rbx_dom_weak::types::Ref;
use thiserror::Error;

/// Represents an error that occurred during serialization.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    source: Box<InnerError>,
}

impl From<InnerError> for Error {
    fn from(inner: InnerError) -> Self {
        Self {
            source: Box::new(inner),
        }
    }
}

#[derive(Debug, Error)]
/// Represents an error that occurred during serialization.
pub enum InnerError {
    /// An I/O error occurred during serialization.
    #[error(transparent)]
    Io {
        /// The underlying I/O error.
        #[from]
        source: io::Error,
    },

    /// A property type mismatch occurred during serialization.
    #[error(
        "Property type mismatch: Expected {type_name}.{prop_name} to be of type {valid_type_names}, \
        but it was of type {actual_type_name} on instance {instance_full_name}",
    )]
    PropTypeMismatch {
        /// The name of the type that contains the property.
        type_name: String,
        /// The name of the property that has a type mismatch.
        prop_name: String,
        /// The valid type names for this property.
        valid_type_names: &'static str,
        /// The actual type name that was encountered.
        actual_type_name: String,
        /// The full name of the instance with the type mismatch.
        instance_full_name: String,
    },

    /// An unsupported property type was encountered during serialization.
    #[error("Unsupported property type: {type_name}.{prop_name} is of type {prop_type}")]
    UnsupportedPropType {
        /// The name of the type that contains the property.
        type_name: String,
        /// The name of the property with the unsupported type.
        prop_name: String,
        /// The unsupported property type.
        prop_type: String,
    },

    /// An invalid property value was encountered during serialization.
    #[error(
        "Invalid property value: The instance {instance_full_name} had a property \
        ({type_name}.{prop_name}) of type {prop_type} with a value that could \
        not be written."
    )]
    InvalidPropValue {
        /// The full name of the instance with the invalid property value.
        instance_full_name: String,
        /// The name of the type that contains the property.
        type_name: String,
        /// The name of the property with the invalid value.
        prop_name: String,
        /// The type of the property with the invalid value.
        prop_type: String,
    },

    /// An invalid instance ID was encountered during serialization.
    #[error("The instance with referent {referent:?} was not present in the dom.")]
    InvalidInstanceId {
        /// The referent that was not found in the DOM.
        referent: Ref,
    },
}
