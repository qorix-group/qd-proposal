==============================================
Proposal: Unified Strategy for AUTOSAR Releases
==============================================

Author       : Suresh Chamuah, PPM
Version      : 1.0  
Date         : 2025-06-18  
Applies To   : All AUTOSAR Adaptive middleware modules (e.g., ComStack, DCM, EM, PHM etc)  
Scope        : R20-11 through R23-11  
Status       : Draft

-------------------------
1. Versioning & Branching
-------------------------

**Objective**: Avoid per-spec branching and ensure a single maintainable codebase for multiple AUTOSAR releases.

**Strategy**:

- Maintain a single `main` or `dev` branch
- Tag AUTOSAR compliance with Git tags:
  
  .. code-block:: bash

     git tag AUTOSAR-R22-11
     git tag AUTOSAR-R23-11

- Create release branches only for major LTS deliveries or customer forks:
  
  .. code-block:: bash

     git checkout -b release/2.0

**Artifact Naming Convention**:

.. code-block:: text

   <ArtifactName>_R<YY-11>_v<MAJOR.MINOR.PATCH>
   e.g., ComStack_R23-11_v2.0.0

----------------------------
2. Compatibility Management
----------------------------

**Objective**: Deliver a single codebase compatible with multiple AUTOSAR releases using conditional logic and configuration.

**Techniques**:

- Define AUTOSAR version through build flags (e.g., `AUTOSAR_VERSION=R23-11`)
- Use configuration files (`config.yaml`) or compile-time defines:

  .. code-block:: c

     #if AUTOSAR_VERSION == 2311
         enableMulticastRouting();
     #endif

- Maintain artifact metadata:

  .. code-block:: yaml

     artifact: ComStack
     version: 2.1.0
     autosar_compatibility:
       - R20-11
       - R21-11
       - R23-11

----------------------------------------
3. Test Coverage Across AUTOSAR Versions
----------------------------------------

**Problem**: Conditional logic results in some code paths not being executed in every build.

**Solution**:

- Execute coverage separately per AUTOSAR spec:

  .. code-block:: bash

     make clean && make AUTOSAR_VERSION=R22-11 COVERAGE=ON
     ./run_tests && gcovr -r . -o coverage_r22-11.json

     make clean && make AUTOSAR_VERSION=R23-11 COVERAGE=ON
     ./run_tests && gcovr -r . -o coverage_r23-11.json

- Merge reports using tools like `gcovr`, `lcov`, or `SonarQube`:

  .. code-block:: bash

     gcovr --add-tracefile coverage_r22-11.json --add-tracefile coverage_r23-11.json -o merged_coverage.json

- Maintain per-spec and combined coverage in the release manifest.

-------------------------------------
4. Documentation & Work Product Reuse
-------------------------------------

**Objective**: Maintain a single SRS, SDS, FTS, and design artifact for all supported AUTOSAR variants.

**Best Practices**:

- Use conditional tagging in `.rst`, `.md`, or `.yaml` files.

  .. code-block:: rst

     .. feat_req::
        :id: COMSTACK_CFG_001
        :autosar: R22-11, R23-11

        ComStack shall support dynamic SOME/IP routing.

- For test cases, use variant-aware tagging:

  .. code-block:: yaml

     id: TC_COM_023
     description: Verify SOME/IP multicast
     autosar_version: R23-11
     variant: PlatformX

- Generate filtered documentation views:

  - `SRS_R23-11.rst`: Auto-generated from master SRS by filtering `:autosar: R23-11`
  - `FTS_CustomerX.xlsx`: Filtered via `variant: CustomerX`

- Maintain a structured folder layout:

  .. code-block:: text

     docs/
       srs/
         master_srs.rst
       sds/
         design_base.rst
       fts/
         test_catalog.yaml
       coverage/
         coverage_r22-11.json
         coverage_r23-11.json
       exports/
         SRS_R23-11.pdf
         SDS_R22-11.docx

-----------------------------------------
5. Build System & Automation Environment
-----------------------------------------

**Toolchain Components**:

- **Build**: CMake + Conan (per AUTOSAR version via `-o autosar_version=Rxx-11`)
- **Coverage**: gcovr / lcov / SonarQube
- **CI**: GitLab CI or GitHub Actions matrix builds
- **Documentation**: Sphinx, PlantUML, Jinja2 for filtered document views
- **Requirements**: reStructuredText, ReqIF, or Excel

**Example Conanfile Snippet**:

.. code-block:: python

   options = {
       "autosar_version": ["R20-11", "R21-11", "R22-11", "R23-11"]
   }

   def build(self):
       cmake.definitions["AUTOSAR_VERSION"] = self.options.autosar_version

--------------------------
6. Summary Recommendations
--------------------------

- **Do not** branch per AUTOSAR spec — tag instead.
- **Design** for configuration-driven compatibility.
- **Measure coverage per spec** and merge reports.
- **Write single-source work products** with variant-aware tagging.
- **Automate documentation and test exports** using filters.
- **Maintain all metadata in YAML or structured formats** for traceability.

------------------
7. Action Items
------------------

- [ ] Migrate legacy per-spec branches to tags
- [ ] Establish variant-aware templates for SRS/SDS/FTS
- [ ] Integrate multi-version coverage merge into CI
- [ ] Define and publish `compat.yaml` for each module
- [ ] Review documentation generators for filtered export capability

