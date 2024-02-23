# What is a "project templating system"?

If you've ever been working on a project and then required some or all of the same project files and structures in another similar project, then you know how annoying it can be to manually copy across all the necessary files and then modify them to suit your new project.

Wouldn't it be nice if something automated this for you?

That's where Zat comes in:
- Create a Zat repository to represent the common files and folder structures across your project.
- Tokenise any differences that are project-specific.
- Use Zat to process the repository and generate a unique versions of the project at a new destination.
