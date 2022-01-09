import {
  commands,
  ExtensionContext,
  languages,
  Position,
  Range,
  TextEdit,
  window,
  workspace,
  WorkspaceEdit,
} from "vscode";
import { execSync } from "child_process";

export function activate(context: ExtensionContext) {
  console.log("Gobelin-vscode: activated");
  formatter(context, formatFile);
  command(context, "gobelin-vscode.format", formatFile);
  command(context, "gobelin-vscode.update", updateAllProject);
  command(context, "gobelin-vscode.add.month", async () => {
    const today = new Date();
    const yearAndMonth = await window.showInputBox({
      value: `${today.getFullYear()} ${
        today.getMonth() < 9 ? `0${today.getMonth() + 1}` : today.getMonth() + 1
      }`,
      valueSelection: undefined,
      placeHolder: "Enter year and month you want. (exp: 2021 12)",
    });
    if (yearAndMonth) {
      addMonth(yearAndMonth);
    }
  });
}

export function deactivate() {}

function formatFile(path: string): string | undefined {
  return runGobelin(`fmt ${path} --stdout`);
}

function updateAllProject(): string | undefined {
  runGobelin(`update --verbose`);
  return undefined;
}

function addMonth(yearAndMonth: string): string | undefined {
  runGobelin(`add month ${yearAndMonth} --verbose`);
  return undefined;
}

function runGobelin(cmd: string): string | undefined {
  const root = workspace.workspaceFolders?.[0].uri.path;
  const gobelinCmd = `/usr/bin/gobelin ${root} ${cmd}`;
  try {
    const stdout = execSync(gobelinCmd, { encoding: 'utf8' });
    console.error("Done:", stdout);
    return stdout;
  } catch (stderr) {
    console.error("Something goes wrong:", stderr);
  }
}

type CurrentFilePathCallback = (currentFilePath: string) => string | Promise<void> | void;

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
      const file = cb(activeTextEditor.document.uri.path);
      if (typeof file === 'string') {
        const document = activeTextEditor.document;
        const lastLine = document.lineAt(document.lineCount-1);
        const fullDocumentRange = new Range(new Position(0,0),new Position(document.lineCount-1,lastLine.text.length));
        const edit = new WorkspaceEdit();
        edit.replace(document.uri, fullDocumentRange, file);
        return workspace.applyEdit(edit);
      }
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
        const file = cb(document.uri.path);
        if (typeof file === 'string') {
          const lastLine = document.lineAt(document.lineCount-1);
          const fullDocumentRange = new Range(new Position(0,0),new Position(document.lineCount-1,lastLine.text.length));
          return [TextEdit.replace(fullDocumentRange, file)];
        } else {
          return [];
        }
      },
    }
  );

  context.subscriptions.push(formatter);
}
