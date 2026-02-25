/// AWS Control Tower lifecycle event definitions.
///
/// Control Tower lifecycle events are delivered via EventBridge as CloudTrail
/// service events with detail-type `"AWS Service Event via CloudTrail"` and
/// source `"aws.controltower"`.
///
/// Use with the `CloudWatchEvent` or `EventBridgeEvent` wrapper:
/// ```ignore
/// use aws_lambda_events::cloudwatch_events::CloudWatchEvent;
/// use aws_lambda_events::controltower::ControlTowerLifecycleEvent;
///
/// let event: CloudWatchEvent<ControlTowerLifecycleEvent> = serde_json::from_slice(data)?;
/// ```
///
/// See <https://docs.aws.amazon.com/controltower/latest/userguide/lifecycle-events.html>
#[cfg(feature = "builders")]
use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The CloudTrail service event delivered as the EventBridge `detail` payload
/// for Control Tower lifecycle events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlTowerLifecycleEvent {
    pub event_version: String,
    pub user_identity: UserIdentity,
    pub event_time: String,
    pub event_source: String,
    pub event_name: String,
    pub aws_region: String,
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: String,
    pub user_agent: String,
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub read_only: bool,
    pub event_type: String,
    pub service_event_details: ServiceEventDetails,
    #[serde(default)]
    pub management_event: Option<bool>,
    #[serde(default)]
    pub recipient_account_id: Option<String>,
    #[serde(default)]
    pub request_parameters: Option<Value>,
    #[serde(default)]
    pub response_elements: Option<Value>,
    #[serde(default)]
    pub event_category: Option<String>,
}

/// User identity information for Control Tower lifecycle events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentity {
    pub account_id: String,
    #[serde(default)]
    pub invoked_by: Option<String>,
}

/// The service event details for Control Tower lifecycle events.
///
/// Each variant corresponds to a specific lifecycle event type.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ServiceEventDetails {
    CreateManagedAccountStatus(ManagedAccountStatus),
    UpdateManagedAccountStatus(ManagedAccountStatus),
    EnableGuardrailStatus(GuardrailStatus),
    DisableGuardrailStatus(GuardrailStatus),
    SetupLandingZoneStatus(LandingZoneStatus),
    UpdateLandingZoneStatus(LandingZoneStatus),
    RegisterOrganizationalUnitStatus(OrganizationalUnitRegistrationStatus),
    DeregisterOrganizationalUnitStatus(OrganizationalUnitRegistrationStatus),
    PrecheckOrganizationalUnitStatus(PrecheckOrganizationalUnitStatus),
    EnableBaselineStatus(BaselineStatus),
    ResetEnabledBaselineStatus(BaselineStatus),
    UpdateEnabledBaselineStatus(BaselineStatus),
    DisableBaselineStatus(BaselineStatus),
}

/// An organizational unit reference.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationalUnit {
    pub organizational_unit_name: String,
    pub organizational_unit_id: String,
}

/// An account reference.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_name: String,
    pub account_id: String,
}

/// A guardrail (control) reference.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guardrail {
    pub guardrail_id: String,
    pub guardrail_behavior: String,
}

/// Status for `CreateManagedAccount` and `UpdateManagedAccount` events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedAccountStatus {
    pub organizational_unit: OrganizationalUnit,
    pub account: Account,
    pub state: String,
    pub message: String,
    pub requested_timestamp: String,
    pub completed_timestamp: String,
}

/// Status for `EnableGuardrail` and `DisableGuardrail` events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardrailStatus {
    pub organizational_units: Vec<OrganizationalUnit>,
    pub guardrails: Vec<Guardrail>,
    pub state: String,
    pub message: String,
    pub request_timestamp: String,
    pub completed_timestamp: String,
}

/// Status for `SetupLandingZone` and `UpdateLandingZone` events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LandingZoneStatus {
    pub state: String,
    pub message: String,
    pub root_organizational_id: String,
    pub organizational_units: Vec<OrganizationalUnit>,
    pub accounts: Vec<Account>,
    pub requested_timestamp: String,
    pub completed_timestamp: String,
}

/// Status for `RegisterOrganizationalUnit` and `DeregisterOrganizationalUnit` events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationalUnitRegistrationStatus {
    pub state: String,
    pub message: String,
    pub organizational_unit: OrganizationalUnit,
    pub requested_timestamp: String,
    pub completed_timestamp: String,
}

/// An organizational unit with precheck failure information.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecheckOrganizationalUnit {
    pub organizational_unit_name: String,
    pub organizational_unit_id: String,
    #[serde(default)]
    pub failed_prechecks: Vec<String>,
}

/// An account with precheck failure information.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecheckAccount {
    pub account_name: String,
    pub account_id: String,
    #[serde(default)]
    pub failed_prechecks: Vec<String>,
}

/// Status for `PrecheckOrganizationalUnit` events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecheckOrganizationalUnitStatus {
    pub organizational_unit: PrecheckOrganizationalUnit,
    pub accounts: Vec<PrecheckAccount>,
    pub state: String,
    pub message: String,
    pub requested_timestamp: String,
    pub completed_timestamp: String,
}

/// Status summary for a baseline operation.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineStatusSummary {
    pub last_operation_identifier: String,
    pub status: String,
}

/// A parameter value wrapping an untyped object.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineParameterValue {
    pub untyped: BaselineUntypedValue,
}

/// An untyped baseline parameter value.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineUntypedValue {
    pub object: String,
}

/// A baseline parameter key-value pair.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineParameter {
    pub key: String,
    pub value: BaselineParameterValue,
}

/// Details about an enabled baseline.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnabledBaselineDetails {
    pub arn: String,
    pub parent_identifier: String,
    pub target_identifier: String,
    pub baseline_identifier: String,
    pub baseline_version: String,
    pub status_summary: BaselineStatusSummary,
    #[serde(default)]
    pub parameters: Vec<BaselineParameter>,
}

/// Status for `EnableBaseline`, `ResetEnabledBaseline`,
/// `UpdateEnabledBaseline`, and `DisableBaseline` events.
///
/// The `baseline_details` field is only present in `DisableBaseline` events.
#[non_exhaustive]
#[cfg_attr(feature = "builders", derive(Builder))]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineStatus {
    pub enabled_baseline_details: EnabledBaselineDetails,
    #[serde(default)]
    pub baseline_details: Option<EnabledBaselineDetails>,
    pub requested_timestamp: String,
    pub completed_timestamp: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_controltower_create_managed_account() {
        let data = include_bytes!("../../fixtures/example-controltower-create-managed-account.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "CreateManagedAccount");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::CreateManagedAccountStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_enable_guardrail() {
        let data = include_bytes!("../../fixtures/example-controltower-enable-guardrail.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "EnableGuardrail");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::EnableGuardrailStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_setup_landing_zone() {
        let data = include_bytes!("../../fixtures/example-controltower-setup-landing-zone.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "SetupLandingZone");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::SetupLandingZoneStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_register_organizational_unit() {
        let data = include_bytes!("../../fixtures/example-controltower-register-organizational-unit.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "RegisterOrganizationalUnit");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::RegisterOrganizationalUnitStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_precheck_organizational_unit() {
        let data = include_bytes!("../../fixtures/example-controltower-precheck-organizational-unit.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "PrecheckOrganizationalUnit");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::PrecheckOrganizationalUnitStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_enable_baseline() {
        let data = include_bytes!("../../fixtures/example-controltower-enable-baseline.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "EnableBaseline");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::EnableBaselineStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_disable_baseline() {
        let data = include_bytes!("../../fixtures/example-controltower-disable-baseline.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "DisableBaseline");
        if let ServiceEventDetails::DisableBaselineStatus(ref status) = parsed.service_event_details {
            assert!(status.baseline_details.is_some());
        } else {
            panic!("Expected DisableBaselineStatus");
        }
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_update_managed_account() {
        let data = include_bytes!("../../fixtures/example-controltower-update-managed-account.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "UpdateManagedAccount");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::UpdateManagedAccountStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_disable_guardrail() {
        let data = include_bytes!("../../fixtures/example-controltower-disable-guardrail.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "DisableGuardrail");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::DisableGuardrailStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_update_landing_zone() {
        let data = include_bytes!("../../fixtures/example-controltower-update-landing-zone.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "UpdateLandingZone");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::UpdateLandingZoneStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_deregister_organizational_unit() {
        let data = include_bytes!("../../fixtures/example-controltower-deregister-organizational-unit.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "DeregisterOrganizationalUnit");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::DeregisterOrganizationalUnitStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_reset_enabled_baseline() {
        let data = include_bytes!("../../fixtures/example-controltower-reset-enabled-baseline.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "ResetEnabledBaseline");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::ResetEnabledBaselineStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn example_controltower_update_enabled_baseline() {
        let data = include_bytes!("../../fixtures/example-controltower-update-enabled-baseline.json");
        let parsed: ControlTowerLifecycleEvent = serde_json::from_slice(data).unwrap();
        assert_eq!(parsed.event_name, "UpdateEnabledBaseline");
        assert!(matches!(
            parsed.service_event_details,
            ServiceEventDetails::UpdateEnabledBaselineStatus(_)
        ));
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ControlTowerLifecycleEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
