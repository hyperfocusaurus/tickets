# Tickets

Tickets is a very simple open source ticketing tool with the following basic features:

* Tickets can be created by web interface, email, or git
* Tickets can be closed by web interface or git
* Tickets are stored in a plain-text markdown format, for maximum interoperability with other tools, and minimum
  external dependencies.  All you need is a filesystem.  Even git is optional.
* Three basic web views on tickets:
  * List view: view all tickets, allowing filtering by label/tag/etc.
  * Tree view: view all tickets in tree form (more on this later)
  * Board view: view all tickets on a kanban style board

# Storage

Tickets are stored in a filesystem in the following manner:

* In the root directory of each status (see more below), tickets named "[ticket].md" -- for example, "user-login.md".  Whitespace in ticket names is
  allowed, but discouraged.  You can use ticket titles to change how the ticket is displayed by the application. 
* In subdirectories, each subdirectory being named after a ticket -- e.g "[ticket]/[other-ticket].md".  These can be
  infinitely nested, to give a rich hierarchy of tickets if that is how you prefer to work.  When creating a sub-ticket,
  the software (or you, if you're doing it manually via git) will create a directory with the ticket name (without the
  .md) and then move the .md file into the directory.  The ticket keeps all of its existing metadata.  If you have a
  folder without a ticket with a matching name inside, the software will first look for `index.md` and `default.md`,
  then look one directory up for a matching named .md file there, before finally giving up and assuming the ticket has
  no metadata yet.  If you edit the ticket, the file will be created and populated automatically.

# Ticket file format

The ticket file format is a basic markdown format that uses the following quirks/tweaks for interpreting the metadata:

The first level 1 heading (single `#` character) will become the title.

The next line after the title is a comma-separated list of tags (aka labels).

The following lines are a colon-separated list of key-value fields.  For example, "priority: critical", or "due-date:
10/11/2023".  The software will try to "guess" at the type of field this is, falling back on string (text) if nothing
else fits.

Separating the colon-separated list from the rest of the file is a single empty newline.

The rest of the file goes in the "Description" field and can use whatever markup you want.

# Backlog, In Progress, Done statuses

To mark an item as "in progress", simply move it to the root directory "in-progress", similar for "done".  If you wish
to delete an item, just delete it from the filesystem.  The web interface provides shortcuts for these tasks.

You can make any number of statuses you want, they will show up as columns in the board, unless they are in the
"exclude-statuses" list of the config file.  You can use this to hide tickets that have been released, by moving them to
a "released" status, or to just hide all "done" tickets if that's how you prefer to work.


# Linking issues together

If a field in an issue is detected to be a relative filepath either from the project root, or from the directory the ticket
file is in, the field will be considered a "link" and a hyperlink will be generated in the web view to the other ticket.

This can be used for e.g "blocked-by: ./foo-bug.md", or "relates-to: bar-bug.md".  Note that you can omit the "status"
part of the path, as each status will be searched for the ticket anyway.  If two statuses have the same ticket in them,
the one which is lexicographically first will be used.  For this reason, you may wish to number your statuses, e.g
"00-todo", "10-inprogress" etc.  You can change the title in the web view by creating a ".status" file with "title: Foo"
inside.



