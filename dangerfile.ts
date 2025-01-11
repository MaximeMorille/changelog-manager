import { danger, fail, markdown, warn } from "danger"

export default async () => {
    const hasChangelog = danger.git.modified_files.includes("CHANGELOG.md");
    const hasNewChangelogEntry = danger.git.fileMatch("unreleased_changelogs/*.md").created;

    if (hasChangelog) {
        fail("Please do not update the CHANGELOG.md file directly. Add a new entry to the unreleased_changelogs folder instead.");
    }

    if (!hasNewChangelogEntry) {
        fail("Please add a new entry to the unreleased_changelogs folder.");
    }
}
