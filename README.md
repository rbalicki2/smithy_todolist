# smithy_todolist

> An example app that uses Smithy to build a todolist that talks to an API.

See it in action [here](https://http://smithy-todolist.robertbalicki.com)!

## How to install and run locally

Clone the following repositories into subfolders of the same folder:

* `git@github.com:rbalicki2/smithy_todolist.git` (this one)
* `git@github.com:rbalicki2/smithy_todolist_server.git`
* `git@github.com:rbalicki2/smithy-app-server`

### Start the server

In the `smithy_todo_list_server`, run

```sh
yarn
npm run start
```

The todo list will now be served at `localhost:3000`.

### Start the app

In the `smithy-app-server` folder, open two terminals and run

```sh
TARGET=../my-smithy-project/ npm run serve
```

and

```sh
TARGET=../my-smithy-project/ npm run watch
```

And now, navigate to `localhost:8080`!
