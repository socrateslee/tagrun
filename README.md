# tagrun

Run a program with a modified process name. You can add a tag for a process with `tagrun`, and use the tag to locate the process with ps, pgrep or pkill.

## Usage
tagrun [--tag PROCESS_NAME_TAG] [--prefix PROCESS_NAME_PREFIX] COMMAND [ARG] ...

Run COMMAND, and rename the process(i.e., argv[0] of the command line). 

    --tag       Rename the command with PROCESS_NAME_TAG.
    --prefix    Prepend the PROCESS_NAME_PREFIX to the command line.

## Example

    tagrun --tag awake --prefix [test] sleep 1000

Above command will be displayed in the result of `ps aux` like

    [test]awake 1000

And you can use `pgrep -f -x -a '\[test\]awake 1000'` to match the process.
