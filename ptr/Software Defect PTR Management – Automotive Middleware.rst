Software Defect PTR Management – Automotive Middleware
=======================================================

1. Purpose
----------

To define the standard process for identifying, tracking, analyzing, fixing, and closing software defects via Problem Tracking Reports (PTRs) in automotive middleware projects.

2. Scope
--------

Applicable to all middleware components (e.g., ``ara::com``, ``ara::log``, SOME/IP, Diagnostics) developed for AUTOSAR Adaptive platforms across all product lines.

3. Roles & Responsibilities
----------------------------

.. list-table:: Roles and Responsibilities
   :header-rows: 1
   :widths: 25 75

   * - Role
     - Responsibility
   * - PTR Coordinator
     - Create, assign, and track PTRs across teams
   * - Developer
     - Analyze root cause, implement fixes
   * - QA/Validation
     - Verify defect fix, run regression tests
   * - Project Manager
     - Ensure fix is delivered in planned release/PI
   * - Customer SPOC
     - Interface with OEMs/Tier-1s and provide updates


4. Tools
--------

- **PTR System**: JIRA / IBM Rational / Polarion
- **Source Control**: Git / Gerrit
- **CI/CD**: Jenkins / Bamboo
- **Test Reports**: Squish, CANoe, Robot Framework

5. Process Steps
----------------

5.1 Defect Logging
~~~~~~~~~~~~~~~~~~

- **Trigger**: Internal test failure or customer report
- Create PTR with:
  - Affected component
  - HW/SW environment
  - Log traces, error codes, screenshots (if available)
  - Severity & priority

5.2 Triage
~~~~~~~~~~

- PTR team reviews within 48 hours
- Assign to relevant module owner
- Add appropriate tags (e.g., ``[Customer]``, ``[Critical Path]``, ``[Integration Blocker]``)

5.3 Analysis
~~~~~~~~~~~~

- Root cause analysis within 3 business days
- Update PTR with diagnosis and proposed resolution

5.4 Fix Implementation
~~~~~~~~~~~~~~~~~~~~~~

- Fix is implemented and linked to PTR ID
- Code submitted via standard review process

5.5 Validation
~~~~~~~~~~~~~~

- Fix is verified via automated and manual tests
- Regression tests executed to ensure stability

5.6 Customer Communication
~~~~~~~~~~~~~~~~~~~~~~~~~~

- PTR status shared in OEM/Tier-1 sync calls
- Early patch delivered if necessary

5.7 Closure
~~~~~~~~~~~

- PTR is closed once verified in release build
- Validation logs and traceability attached

6. Metrics to Track
-------------------

- Average time to triage
- Average time to resolution
- Percentage of defects reopened
- PTRs per module/component
- Internal vs. customer-reported defect ratio

7. Review & Audits
-------------------

- Monthly review of defect trends with System Architect and Quality Lead
- Quarterly audit of closed PTRs for completeness and traceability

8. Diagram – Defect Lifecycle
-----------------------------

.. code-block:: text

   +--------------------+
   | Defect Identified  |
   | (Test/Customer)    |
   +--------+-----------+
            |
            v
   +--------+-----------+
   | Log PTR in System  |
   | (JIRA/Polarion)    |
   +--------+-----------+
            |
            v
   +--------+-----------+
   | Triage & Categorize|
   +--------+-----------+
            |
            v
   +---------------------+
   | Assign to Module    |
   | Owner / Developer   |
   +---------------------+
            |
            v
   +---------------------+
   | Root Cause Analysis |
   +---------------------+
            |
            v
   +---------------------+
   | Fix Implementation  |
   +---------------------+
            |
            v
   +---------------------+
   | Integration &       |
   | Regression Testing  |
   +---------------------+
            |
            v
   +---------------------+
   | PTR Status Update   |
   | to Stakeholders     |
   +---------------------+
            |
            v
   +---------------------+
   | Validation Sign-off |
   +---------------------+
            |
            v
   +---------------------+
   | Close PTR & Archive |
   +---------------------+

9. Cadence and Bandwidth Allocation per PI
-------------------------------------------

9.1 Cadence
~~~~~~~~~~~

**Defect Review Meetings:**
- Weekly triage meetings (e.g., every Tuesday)
- Biweekly customer-facing PTR reviews
- Monthly defect analysis with Quality Lead and System Architect

**PTR Status Reports:**
- Shared every Friday internally
- PI summary shared during System Demo and Inspect & Adapt

**Dashboard Updates:**
- Weekly PTR updates in JIRA/Polarion dashboards

9.2 Bandwidth Allocation Guidelines
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table:: Bandwidth Allocation Per PI Phase
   :header-rows: 1
   :widths: 30 40 30

   * - PI Phase
     - Activity
     - Suggested Allocation (%)
   * - PI Planning
     - Review backlog PTRs, estimation
     - 10%
   * - Execution Weeks 1–6
     - Active defect analysis/fixing
     - 15–25% (per developer)
   * - System Demo Week
     - Regression, closure
     - 5–10%
   * - Innovation/Buffer Week
     - Root cause backlog cleanup
     - 5–10%

.. note::

   📌 **Note**: Bandwidth may increase in stabilization-heavy PIs. Adjustments are coordinated with PM and RTE based on defect trend and program phase.


.. note::

   📌 **Note**: Bandwidth may increase in stabilization-heavy PIs. Adjustments are coordinated with PM and RTE based on defect trend and program phase.

9.3 Planning Inputs
~~~~~~~~~~~~~~~~~~~

- Defect Severity & Volume from past 2 PIs
- Open High/Critical PTRs
- Customer escalation status
- Component maturity and stability

9.4 Resource Assignment
~~~~~~~~~~~~~~~~~~~~~~~

- **Dedicated PTR Owners** for core middleware modules
- **Rotational Triage Engineer** each PI
- **Shared QA Validation Pool** for fix testing and regression
