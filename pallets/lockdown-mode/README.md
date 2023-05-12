# Lockdown Mode Pallet

The Lockdown Mode Pallet is a Substrate module that provides functionality to lock down the runtime execution in a Substrate-based blockchain system. When the lockdown mode is activated, it filters out incoming calls and messages to ensure that only authorized actions are allowed.

## Overview

This pallet the governance of the chain to activate or deactivate a lockdown mode. When the lockdown mode is activated, incoming runtime calls and downward messages are filtered based on a preconfigured filter. Additionally, it suspends the execution of XCM (Cross-Consensus Message) messages in the `on_idle` hook.

The lockdown mode status is stored in the `LockdownModeStatus` storage item. When the lockdown mode is deactivated, the system resumes normal operations, including the execution of XCM messages in the `on_idle` hook.

## Configuration

This pallet supports configurable traits that allow customization according to specific needs.

### Types

- `RuntimeEvent`: Specifies the runtime event type.
- `LockdownModeOrigin`: Specifies the origin that is allowed to activate and deactivate the lockdown mode.
- `BlackListedCalls`: Specifies the filter used to filter incoming runtime calls in lockdown mode.
- `LockdownDmpHandler`: Specifies the handler for downward messages in lockdown mode.
- `XcmExecutorManager`: Interface to control the execution of XCMP Queue messages.


## Extrinsics

The pallet provides the following extrinsics:

- `activate_lockdown_mode`: Activates the lockdown mode. Only the specified `LockdownModeOrigin` can call this extrinsic. It updates the `LockdownModeStatus` storage item to `ACTIVATED` (true) and attempts to suspend the execution of XCM messages in the `on_idle` hook.
- `deactivate_lockdown_mode`: Deactivates the lockdown mode. Only the specified `LockdownModeOrigin` can call this extrinsic. It updates the `LockdownModeStatus` storage item to `DEACTIVATED` (false) and attempts to resume the execution of XCM messages in the `on_idle` hook.


#### Errors

Possible errors returned by the dispatchable calls are:

- `LockdownModeAlreadyActivated`: The lockdown mode is already activated.
- `LockdownModeAlreadyDeactivated`: The lockdown mode is already deactivated.
  
Please note that any failure to suspend or resume XCM execution in the `on_idle` hook is not treated as a fatal error that stops the function execution. Instead, it is recorded as an event `FailedToSuspendIdleXcmExecution` or `FailedToResumeIdleXcmExecution`, respectively, and the function continues its execution.

The lockdown mode can serve as a crucial tool in system maintenance or in case of emergency, when it's necessary to restrict system operation and ensure the system's security and stability.
