
# CODE DOCUMENTATION

## What is this?

Pupik editor is a structure editor for my standalone game Pupik written in pure Rust using the Bevy game engine (version 0.11), which doesn't have a graphic editor as of today (31.08.2023), so everything is code.

In Pupik, there are random objects falling from the sky that the player can collect/dodge/ignore. While random generation was fine as it was, there was so much missed potential in synergizing these objects and thats what this program is for. Here one can build different objects, export their relative coordinates and then import them into the game by putting them in the right folder.

Because the folder for both importing and exporting is named "structures", you can move all dlls that Pupik uses into Pupik-editor's folder, combine the assets in the "assets" folder and then have both executables in the same folder. This way, you dont have to move the objects between folders after creating, they are automatically loaded in upon starting the game.


## Used libraries, engines and assets

I chose Bevy simply because the original game was made in Bevy, so not only I already knew how to work with it, but also if I decided to merge these programs later on, I could just... do it.

I also used bevy_egui 0.21, but only for text input. I planned on making all the GUI myself, but I didn't have time to implement my own text fields, so I used the library just for this. Once I return to this project, I will probably use it either for everything or for nothing.

Most assets are self made or at least editted by me personally. 

## Short bevy description

Bevy game engine uses Entity Component System, where Entities are unique things that are assigned groups of Components, which are then processed using Systems. In Bevy one filters entities based on their components using Queries<> (there you can specify which fields does the entity have using With<> or doesnt have using Without<> and more), accesses resources through Res(Mut) keyword and spawns/despawns entities using Commands.

## Some technical details:

THIS PROGRAM MAY OR MAY NOT WORK ON WINDOWS 11. THE ONLY TESTED OS IS WINDOWS 10.

I decided to split the game into multiple modules, and if they have a System that should be called by the app, the module also includes a similarly named plugin.

The game shouldnt be run in fullscreen and I did my best to prevent it. Fullscreen support is probably my next thing on TODO list tho.

Placing the gingerbread circle means a random basic enemy will spawn in the game. Not necessarily the gingerbread.

Placing the plane object actually means the height of a plane sensor, which will spawn a plane upon collision. While I tried to make it so the plane roughly flies through the middle of the sensor, it still could be a bit confusing and im coming up with a less confusing way to do this as you are reading this. 

When you spawn an object, a white button spawns on top of it. This may look weird, but i found it the most convinient in the long run. I, as a developer, can easily read the button event and users will also have easier time targeting the correct object. These buttons will not be exported.

## Short description of game behaviour

After turning the editor on, all of the spritesheets get loaded in and UI is spawned

User then can choose any of the default 6 game objects and place them in the middle section of the screen. They can also place it behind the transparent side bars, they are just informing the user what will actually be seen in the game. So spawning basic damaging objects will be worthless there.

If the user wants to delete the object, they can select the eraser tool and then click on the white button that corresponds to the soon to be erased object.

Once the user wants to export the structure, they *must* enter its relative weight, the is no default value for that. The bigger the weight, the more often it will spawn. I couldn't come up with a more straight forward way to do this. Inputing odds of spawning in percentages is impossible, because, simply put, if you keep on making structures with 50% odds of spawning, it can't be 50% for all. I could recalculate them, but then the input value would behave very unpredictably.

Name of the structure is by default "export", but different name can be entered. Then they click the checkmark button and the file appears in a folder named "structures".

To exit the editor, one must click the X button in the top left.

After exporting the structure, user moves into the folder with the original game and puts the generated file into a "structures" folder. It may be generated, but user may have to create it by himself.

Then once the user plays the game, structure will be spawning appropriately.

## Modules
A list of all functions implemented in each module can be found in main.rs, here is a quick overview:

export.rs - UI regarding export.

mouse.rs - Everything regarding mouse, like movement, erasing,...

structure_ui.rs - Everything regarding UI.

### Compile with --release flag!!!