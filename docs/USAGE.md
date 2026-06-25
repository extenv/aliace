# Aliace Operational Reference Guide

This reference manual provides comprehensive instructions for administering and executing operations within the Aliace command orchestration platform.

---

## 🖥️ 1. Interactive Terminal User Interface (TUI) Mode

The TUI provides operators with a visual console to manage registered commands, build sequential pipeline groups, inspect execution telemetry, and manage registry backups.

### Console Navigation & Hotkeys

| Hotkey | Action | Description |
| --- | --- | --- |
| `L` | View Registry | Navigate to the tabbed Command and Group list panel |
| `A` | Register Command | Open the wizard to register a new command entry |
| `G` | Register Group | Open the wizard to assemble a command pipeline group |
| `U` | Update Entry | Open the selector to update command or group attributes |
| `D` | Remove Entry | Open the selector to delete command or group entries |
| `E` | Export Database | Open the configuration panel to export backups |
| `I` | Import Database | Open the configuration panel to restore backups |
| `Up` / `Down` | Select Most Run | Navigate selection within the "Most Run Items" dashboard panel |
| `Enter` / `R` | Run Highlighted | Execute the highlighted "Most Run" command or pipeline sequence |
| `Esc` / `Q` | Exit Context | Return to the Dashboard or exit the application console |

---

### Command & Group List Panel

The List Panel displays a dual-pane layout: a master list on the left and a detailed telemetry panel on the right.

* **Context Switching**: Use the `Left` and `Right` arrow keys to switch between the **Single Commands** tab and the **Pipeline Groups** tab.
* **Selection**: Navigate command entries using the `Up` and `Down` arrow keys.
* **Execution**: Press `Enter` or `R` to trigger execution of the selected item.
  * *Note: During execution, TUI rendering is suspended. The process executes directly in the native shell. When execution finishes, you are prompted to press `Enter` to restore TUI graphics.*
* **Modification**: Press `E` to modify the highlighted item within the editor wizard.
* **Deletion**: Press `D` to remove the highlighted item (triggers a confirmation popup modal).

---

### Interactive Editor Wizards

#### Command Registration Wizard Fields:
* **Title**: Unique identifier used to trigger the command via the CLI.
* **Description**: Functional summary of the command's purpose.
* **Script**: The exact shell payload to execute.
* **Group**: Optional metadata tag for logical classification.

* **Navigation**:
  * `Tab` / `Down`: Move cursor focus forward.
  * `Up`: Move cursor focus backward.
  * `Enter` on `Save` / `Cancel`: Commit modifications or abort.

#### Pipeline Group Wizard Fields:
* **Name**: Unique identifier for the pipeline group.
* **Description**: Functional summary of the group's purpose.
* **Sequence List**: Ordered commands forming the execution pipeline.
* **Available Commands**: All registered single commands eligible for addition to the pipeline.

* **Navigation & Order Editing**:
  * `Tab` / `BackTab`: Toggle cursor focus among all form sections.
  * **Within Available Commands Pane**:
    * `Up` / `Down`: Highlight a command.
    * `Enter` or `A`: Append the highlighted command to the sequence list.
  * **Within Sequence List Pane**:
    * `Up` / `Down`: Navigate target sequence positions.
    * `Shift` + `Up` / `Shift` + `Down` (or `K` / `J`): Reorder the selected command up or down the pipeline sequence.
    * `Backspace` / `Delete` (or `D`): Remove the highlighted command from the pipeline sequence.

---

## 🐚 2. Command Line Interface (CLI) & Scripting Mode

The CLI mode allows system administrators to integrate Aliace directly into automated environments, cron jobs, and CI/CD pipelines.

### General Information Commands
Query utility state or request runtime usage assistance:
```bash
# Display binary version
aliace -v

# Display standard help output
aliace -h
# or: aliace --help
```

### Execution Orchestration
Execute any single command or sequential pipeline directly from the shell:
```bash
aliace run <title_or_name>
```
*Pipeline Execution Behavior: When executing a Pipeline Group, Aliace runs each registered command sequentially. If any command exits with a non-zero status code, pipeline orchestration halts immediately to prevent compounding system failures.*

---

### Dynamic Argument Prompting (Enclosed in `<>`)
If a registered script contains template placeholders enclosed in angle brackets (e.g., `<message>`, `<branch>`), Aliace pauses execution and prompts the operator for runtime input:
* **Configured Script**: `git commit -m "<message>"`
* **Runtime Console Flow**:
  ```
  Running command 'commit': git commit -m "<message>"
  Enter value for <message>: Initial release
  ```
  Aliace replaces the template tag with the input string and runs the resolved payload (`git commit -m "Initial release"`). The resolved script is recorded in the execution history.

---

### Database Import & Export
Restore registry data dynamically from a JSON backup file:
```bash
aliace import <path/to/backup.json>
```

---

### Registry Administration (`aliace command` & `aliace group`)

Programmatically manage single command configurations and pipeline sequences:

#### CLI Commands:
```bash
# Add a new command entry
aliace command add --title <title> --script <script> [--desc <desc>] [--group <group>]

# Update attributes of an existing command entry
aliace command update --title <title> [--script <new_script>] [--desc <new_desc>] [--group <new_group>]

# Delete a command entry
aliace command delete --title <title>

# List all command registry entries
aliace command list
```

#### CLI Groups:
```bash
# Add a new pipeline group sequence
aliace group add --name <name> --desc <desc> [--commands <c1,c2,...>]

# Update attributes of an existing pipeline group sequence
aliace group update --name <name> [--desc <new_desc>] [--commands <new_c1,new_c2,...>]

# Delete a pipeline group sequence
aliace group delete --name <name>

# List all pipeline group sequence entries
aliace group list
```
