#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub type ISearchPaneQueryChangedEventArgs = *mut ::core::ffi::c_void;
pub type LocalContentSuggestionSettings = *mut ::core::ffi::c_void;
pub type SearchPane = *mut ::core::ffi::c_void;
pub type SearchPaneQueryChangedEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneQueryLinguisticDetails = *mut ::core::ffi::c_void;
pub type SearchPaneQuerySubmittedEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneResultSuggestionChosenEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneSuggestionsRequest = *mut ::core::ffi::c_void;
pub type SearchPaneSuggestionsRequestDeferral = *mut ::core::ffi::c_void;
pub type SearchPaneSuggestionsRequestedEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneVisibilityChangedEventArgs = *mut ::core::ffi::c_void;
pub type SearchQueryLinguisticDetails = *mut ::core::ffi::c_void;
pub type SearchSuggestionCollection = *mut ::core::ffi::c_void;
pub type SearchSuggestionsRequest = *mut ::core::ffi::c_void;
pub type SearchSuggestionsRequestDeferral = *mut ::core::ffi::c_void;
