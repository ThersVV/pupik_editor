# GAME

Pupik-editor is a small "admin tool" to create simple structures from default objects for my open-source Pupik game. 

## Instalation
0. You will need about 150MB of free space (~45MB for download, ~90MB after extraction).
1. You can download the game at https://drive.google.com/drive/folders/1C00KSZbi6d3-HmDSUqTo4dWiJ1BAnOP6?usp=drive_link .
2. Once download finishes, open the file using any program capable of opening .zip files (I use WinRar), you will see a file named "pupik_editor" inside. 
3. Extract that to wherever you want (in WinRar you can simply drag it outside the window).
4. Open the file you extracted.
5. Run the editor by running pupik_editor.exe.

## How to use it

### Building
You can select and of the objects in the bottom row and place them anywhere except UI, but if you want to actually see them in the game (and the objects don't move horizontally), you will have to place them in the middle section.
All but 2 objects behave how one would expect. 
1. If you place a plane, in the game it will translate to an invisible sensor, that will release a plane once the player touches it.
2. The gingerbread circle won't necessarily spawn a gingerbread circle. It will choose one of the basic enemies (the ones without special efects like gravity) at random.
If you want to erase on object, select the eraser tool and click on the white rectangle, that spawnd together with the object (those white rectangles won't be exported).

### Exporting
Once your structure is ready, enter its file name and its relative weight. What does relative weight mean? It's something like a spawn chance, the bigger the value, the more often it will spawn.
If you forget to enter a file name, it will be exported as "export", but if you forget to enter the weight, **nothing will be exported**!

To close the window, click on the cross button on the top left.

You will find your exported file in the "structures" folder.

### Importing
To import your custom structure to the Pupik game, follow these steps:
1. Copy your structure by right clicking it and selecting "Copy"
2. Move to the folder where Pupik.exe is located
3. Find the "structures" folder. If there isn't any, create it by right clicking somewhere in the folder, selecting "New" and then "Folder"
4. Enter the "structures" folder
5. Paste your custom structure in right clicking and selecting "Paste"
After running the game, the object should be spawning based on the weight you chose.

Only the basic 6 objects and structures located in the "structures" folder will be spawning, once you move your structure out of the "structures" folder, it will no longer spawn.