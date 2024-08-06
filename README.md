## FindAllCompaniesTruck

Script created for the purpose of finding the exact parking spot within the games **Euro Truck Simulator 2**, **American Truck Simulator** and their map modifications.

### How to use?

-   First you will have to start your game in editor mode using the `-edit` startup parameter that you can apply from steam or a shortcut.
-   Once you have the editor open and select the desired map you will have to export the settings from the game console with the command `edit_save_text [any_name]`.
-   Once you have exported all the data (which takes quite some time) you will need to enter the folder in `Documents` of your game, where you will enter the folder `mod` -> `user_map` -> `map` and finally the folder named with the name you gave when you exported the data. From this folder is the path you need to replace in the code for `path`.
