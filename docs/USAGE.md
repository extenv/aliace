# Aliace - Detailed Usage Guide

This guide describes how to operate Aliace via the **Interactive TUI** and the **Terminal CLI Scripting** interfaces.

---

## 🖥️ 1. Interactive TUI Guide

The TUI provides a visual dashboard to manage commands, execute sequences, view execution stats, and perform import/export operations.

### Dashboard & Navigation Shortcuts

| Key | Action |
| --- | --- |
| `L` | Go to Commands & Groups List Screen |
| `A` | Go to Add Single Command Screen |
| `G` | Go to Add Group Sequence Screen |
| `U` | Go to Update/Edit Selection List Screen |
| `D` | Go to Delete Selection List Screen |
| `E` | Go to Export Backup Screen |
| `I` | Go to Import Backup Screen |
| `Esc` / `Q` | Quit / Go back to Dashboard |

---

### Commands & Groups List Screen

The list screen is split horizontally: the left side lists registered items, and the right side displays details of the selected item.

* **Switch Tabs**: Use `Left` or `Right` arrow keys to toggle between **Single** commands and **Group** sequences.
* **Select Item**: Use `Up` or `Down` arrow keys.
* **Run Selected**: Press `Enter` or `R` to run the highlighted command or group.
  * *Note: When running, the TUI suspends, runs the commands in your shell, and prompts you to press `Enter` to return back to the TUI.*
* **Edit Selected**: Press `E` to open the interactive update form.
* **Delete Selected**: Press `D` to delete the item (requires confirmation popup).

---

### Interactive Editor Forms

#### Single Command Form fields:
* **Title**: The name of the command.
* **Description**: Optional description.
* **Script**: The shell command string to execute.
* **Group**: Optional tag to associate it with a group.

* **Form Shortcuts**:
  * `Tab` / `Down`: Move focus to the next field.
  * `Up`: Move focus to the previous field.
  * `Enter` on `Save` / `Cancel` buttons: Submit form or exit.

#### Group Sequence Form fields:
* **Name**: The group name.
* **Description**: Optional description.
* **Sequence List**: Command sequences inside the group, ordered by execution precedence.
* **Available Commands**: All registered single commands that can be added to the sequence.

* **Group Form Shortcuts**:
  * `Tab` / `BackTab`: Switch focus between fields (Name, Description, Sequence List, Available Commands, Save, Cancel).
  * **Inside Available Commands**:
    * `Up` / `Down` to navigate.
    * `Enter` or `A` key to **add** the selected command to the group sequence.
  * **Inside Sequence List**:
    * `Up` / `Down` to navigate the sequence list.
    * `Shift` + `Up` / `Shift` + `Down` (or `K` / `J`) to **reorder/swap position** of the command within the execution order.
    * `Backspace` or `Delete` (or `D`) to **remove** the command from the sequence.

---

## 🐚 2. Terminal CLI Scripting Guide

You can script Aliace directly from your shell. All changes update the local registry automatically.

### General Commands (Version & Help)

* **Check Version**:
  ```bash
  aliace -v
  # or: aliace --version
  # or: aliace version
  ```

* **Show Help Text**:
  ```bash
  aliace -h
  # or: aliace --help
  # or: aliace help
  ```

### Running Commands & Groups
Execute any single command or sequential group directly:
```bash
aliace run <title>
```
*If a Group name is passed, Aliace will execute each of its registered commands sequentially. If any command fails (non-zero exit code), execution stops.*

### Importing Database Backups
```bash
aliace import <path/to/backup.json>
```

### Deleting Items
```bash
aliace delete <title>
```

---

### CLI Commands (`aliace command`)

Manage single commands using the following commands:

#### Add Command:
```bash
aliace command add --title <title> --script <script> [--desc <description>] [--group <group_tag>]
```

#### Update Command:
```bash
aliace command update --title <title> [--script <new_script>] [--desc <new_description>] [--group <new_group>]
```

#### Delete Command:
```bash
aliace command delete --title <title>
```

#### List Commands (Prints raw command details):
```bash
aliace command list
```

---

### CLI Groups (`aliace group`)

Manage sequence groups using the following commands:

#### Add Group Sequence:
```bash
aliace group add --name <name> --desc <description> [--commands <c1,c2,c3,...>]
```
*Specify commands as a comma-separated list of existing command titles.*

#### Update Group Sequence:
```bash
aliace group update --name <name> [--desc <new_description>] [--commands <new_c1,new_c2,...>]
```

#### Delete Group Sequence:
```bash
aliace group delete --name <name>
```

#### List Groups:
```bash
aliace group list
```
