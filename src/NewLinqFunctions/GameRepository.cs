namespace NewLinqFunctions;

public static class GameRepository
{
    public static List<Game> Load()
    {
        return [
            new Game("Super Mario Bros", 9.5, "Platform", 1985),
            new Game("The Legend of Zelda", 9.2, "Adventure", 1986),
            new Game("Pac-Man", 8.9, "Arcade", 1980),
            new Game("Quake", 9.0, "FPS", 1996),
            new Game("Diablo", 9.2, "RPG", 1996),
            new Game("Crash Bandicoot", 8.8, "Platform", 1996),
            new Game("Metal Gear Solid", 9.8, "Action", 1998),
            new Game("Tetris", 9.3, "Puzzle", 1984),
            new Game("Donkey Kong", 8.8, "Platform", 1981),
            new Game("Final Fantasy", 9.0, "RPG", 1987),
            new Game("Mega Man", 8.5, "Platform", 1987),
            new Game("Street Fighter II", 9.1, "Fighting", 1991),
            new Game("Sonic the Hedgehog", 8.7, "Platform", 1991),
            new Game("Doom", 9.4, "FPS", 1993),
            new Game("Mortal Kombat", 8.6, "Fighting", 1992),
            new Game("Half-Life", 9.7, "FPS", 1998),
            new Game("StarCraft", 9.6, "Strategy", 1998),
            new Game("Gran Turismo", 9.1, "Racing", 1997),
            new Game("Tomb Raider", 9.0, "Adventure", 1996),
            new Game("Age of Empires", 9.3, "Strategy", 1997),
        ];
    }
}
