# Change Request (CRQ) - Git Procedure: One Commit Per CRQ

*   **CRQ ID:** TBD-006-GIT-PROCEDURE-CRQ-COMMIT
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Process/Procedure Documentation
*   **Description of Change:**
    This CRQ documents a new Git commit procedure: grouping related file changes and their corresponding Change Request (CRQ) documents into a single, atomic commit. Each CRQ will now correspond to a distinct commit, ensuring better traceability, reviewability, and adherence to change management principles. This procedure aims to improve the clarity of the Git history by linking specific code changes directly to their formal documentation.
*   **Reason for Change (Compliance Link):**
    To enhance the project's change management and auditability by establishing a clear, one-to-one relationship between code commits and formal CRQ documentation. This improves transparency and simplifies the process of understanding the rationale and scope of each change.
*   **Impact Assessment:**
    *   **Systems/Services:** Git repository history, change management processes.
    *   **Users:** Developers, reviewers, and auditors interacting with the Git history and CRQ system.
    *   **Potential Risks:** Initial overhead in adapting to the new procedure; potential for misgrouping files if not carefully executed.
*   **Affected Files/Components:**
    *   Git repository commit history.
    *   CRQ documentation files (`docs/crq/`).
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Clear communication and training on the new procedure.
    *   Emphasis on careful staging of files before committing.
    *   Automated checks (if feasible) to ensure CRQ presence for relevant commits.
*   **Rollback Plan:**
    *   Revert to previous commit practices (though this CRQ primarily documents a process, not a code change).
*   **Testing/Verification Plan:**
    *   Verify that subsequent commits adhere to the new one-commit-per-CRQ structure.
    *   Confirm that CRQ documents are correctly linked to their respective commits.
