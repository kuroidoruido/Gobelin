import { commands, ExtensionContext, languages, window, workspace } from "vscode";
import { exec } from 'child_process';

export function activate(context: ExtensionContext) {
  console.log("Gobelin-vscode: activated");
  formatter(context, formatFile);
  command(context, "gobelin-vscode.format", formatFile);
  command(context, "gobelin-vscode.update", updateAllProject);
  command(context, "gobelin-vscode.add.month", async () => {
    const today = new Date();
    const yearAndMonth = await window.showInputBox({
      value: `${today.getFullYear()} ${today.getMonth() < 9 ? `0${today.getMonth()+1}` : today.getMonth()+1}`,
      valueSelection: undefined,
      placeHolder: "Enter year and month you want. (exp: 2021 12)",
    });
	if (yearAndMonth) {
		addMonth(yearAndMonth);
	}
  });
}

export function deactivate() {}

function formatFile(path: string): void {
  runGobelin(`fmt ${path} --verbose`);
}

function updateAllProject(): void {
  runGobelin(`update --verbose`);
}

function addMonth(yearAndMonth: string): void {
  runGobelin(`add month ${yearAndMonth} --verbose`);
}

function runGobelin(cmd: string) {
  const root = workspace.workspaceFolders?.[0].uri.path;
	const gobelinCmd = `/usr/bin/gobelin ${root} ${cmd}`;
	exec(gobelinCmd, (stderr, stdout) => {
    if (stderr) {
      console.error('Something goes wrong:',stderr);
    } else {
      console.error('Done:',stdout);
    }
  });
}

type CurrentFilePathCallback = (currentFilePath: string) => void;

function command(
  context: ExtensionContext,
  commandId: string,
  cb: CurrentFilePathCallback
) {
  const customCommand = commands.registerCommand(commandId, () => {
    const { activeTextEditor } = window;
    if (
      activeTextEditor &&
      activeTextEditor.document.languageId === "gobelin-lang"
    ) {
      cb(activeTextEditor.document.uri.path);
    }
  });

  context.subscriptions.push(customCommand);
}

function formatter(
  context: ExtensionContext,
  cb: CurrentFilePathCallback
): void {
  const formatter = languages.registerDocumentFormattingEditProvider(
    "gobelin-lang",
    {
      provideDocumentFormattingEdits(document) {
        cb(document.uri.path);
        return [];
      },
    }
  );

  context.subscriptions.push(formatter);
}
