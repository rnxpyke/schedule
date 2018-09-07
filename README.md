# schedule

schedule is a tool similar to Trello to manage different projects and tasks.

### filesystem layout

all the of schedule's data is stored in a common `$data` directory, such as `/home/user/.schedule/`
this directory is a listing of all projects the current user is part of. 
A project is a directory with an arbitrary name. 
A project contains a 
- `name` file, which in turn contains the project name
- `tasks` directory, which contains all associated tasks

A task is a directory with arbitrary name.
A task contains a 
- `name` file, which contains the task name
- `group` file, which contains the groups the task is associated with.

Groups are used to order tasks. The standard groups are 
		- `todo` 
		- `in_progress`
		- `finished`
a task can only ever be in one of the standard groups. If more than one standard group is listed, only the first is interpreted as that tasks group.  a task can have any number of non-standard groups. A task without an associated standard group is considered to be in the `misc` group

### cli
there will be a command line program to create, edit and remove projects and tasks.
there will also be a json-rpc mode that exposes the same operations. Additionally, when run in this mode,
the application will hold an in memory grid of projects and tasks.
This way, it will be possible to easily build a graphical frontend for the application. 

