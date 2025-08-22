============================================
End-to-End Protection Properties in Adaptive AUTOSAR (R20-11)
============================================

This document describes the use of ``End2EndEventProtectionProps`` and ``End2EndMethodProtectionProps``
in **single-binding** and **multi-binding** scenarios, with emphasis on the uniqueness rules for ``dataID``.


Overview
========
- **dataID**: Numeric identifier used in CRC calculation to uniquely identify a protected event or method.
- **Purpose**: Ensures receiver can verify that the data belongs to the intended source and not a different protected item.
- **Uniqueness Rule**:
  - Must be **unique per protected event/method** in the system.
  - **Exception**: Duplicate allowed for **multi-binding** (same event/method bound to multiple receivers).


Single-Binding
==============
In a single-binding scenario, one sender is bound to exactly one receiver.

``dataID`` must be unique across all events/methods.

Example: Engine Control Application
-----------------------------------

**Applications**:

- *App_EngineControl* publishes an event ``EngineSpeed``.
- *App_Display* subscribes to ``EngineSpeed``.

**Configuration**:

.. code-block:: xml

   <End2EndEventProtectionProps>
       <DataID>0x1234</DataID>
       <Profile>E2EProfile4</Profile>
   </End2EndEventProtectionProps>

**Visual Diagram**:

.. mermaid::

   graph TD
       A[App_EngineControl] -->|EngineSpeed (DataID=0x1234)| B[App_Display]

Result:
- ``dataID=0x1234`` uniquely identifies ``EngineSpeed``.
- No other event/method in the system may use ``0x1234``.


Multi-Binding
=============
In a multi-binding scenario, one sender is bound to multiple receivers.

``dataID`` duplication is allowed **only** for the *same event/method*, as it represents the same protected data stream.

Example: Engine Control Application with Multiple Consumers
-----------------------------------------------------------

**Applications**:

- *App_EngineControl* publishes an event ``EngineSpeed``.
- *App_Display*, *App_Logger*, and *App_Telematics* subscribe to ``EngineSpeed``.

**Configuration** (for all bindings):

.. code-block:: xml

   <End2EndEventProtectionProps>
       <DataID>0x1234</DataID>
       <Profile>E2EProfile4</Profile>
   </End2EndEventProtectionProps>

**Visual Diagram**:

.. mermaid::

   graph TD
       A[App_EngineControl] -->|EngineSpeed (DataID=0x1234)| B[App_Display]
       A -->|EngineSpeed (DataID=0x1234)| C[App_Logger]
       A -->|EngineSpeed (DataID=0x1234)| D[App_Telematics]

Result:
- All receivers expect ``dataID=0x1234``.
- Allowed duplication since the same event ``EngineSpeed`` is consumed by multiple receivers.


Method Protection Example
==========================

Single-Binding
--------------
**Applications**:

- *App_Diagnostics* provides method ``ReadDTC``.
- *App_Tester* invokes ``ReadDTC``.

**Configuration**:

.. code-block:: xml

   <End2EndMethodProtectionProps>
       <DataID>0x5678</DataID>
       <Profile>E2EProfile5</Profile>
   </End2EndMethodProtectionProps>

**Visual Diagram**:

.. mermaid::

   graph TD
       A[App_Tester] -->|ReadDTC (DataID=0x5678)| B[App_Diagnostics]

Result:
- ``dataID=0x5678`` uniquely identifies method ``ReadDTC``.

Multi-Binding
--------------
**Applications**:

- *App_Diagnostics* provides method ``ReadDTC``.
- *App_Tester*, *App_RemoteDiag*, and *App_CloudDiag* all invoke ``ReadDTC``.

**Configuration** (for all bindings):

.. code-block:: xml

   <End2EndMethodProtectionProps>
       <DataID>0x5678</DataID>
       <Profile>E2EProfile5</Profile>
   </End2EndMethodProtectionProps>

**Visual Diagram**:

.. mermaid::

   graph TD
       A[App_Tester] -->|ReadDTC (DataID=0x5678)| B[App_Diagnostics]
       C[App_RemoteDiag] -->|ReadDTC (DataID=0x5678)| B
       D[App_CloudDiag] -->|ReadDTC (DataID=0x5678)| B

Result:
- ``dataID=0x5678`` is duplicated across all bindings.
- Allowed, since all refer to the *same method* ``ReadDTC``.


Summary
=======
- ``dataID`` must be unique across the system **per protected event/method**.
- **Single-binding**: strict uniqueness enforced.
- **Multi-binding**: duplicate allowed, but only for the *same* event or method instance with multiple consumers.
- Visual diagrams clarify sender-receiver relationships and ``dataID`` flow.

