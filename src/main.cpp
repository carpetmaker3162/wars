#include <pch.hpp>

#include <glad/glad.h>
#include <Game.hpp>
#include <GameOptions.hpp>

using wars::Window;

int main(int argc, char** argv)
{
    // Put the command line arguments into a vector so that it's easier to deal
    // with.
    std::vector<std::string> commandLineArguments(argc);
    for (uint16_t i = 0; i < argc; i++)
    {
        commandLineArguments[i] = argv[i];
    }
    
    // Process the command line arguments
    for (std::vector<std::string>::iterator argument = commandLineArguments.begin(); argument != commandLineArguments.end(); argument++)
    {
        if (*argument == "--asset-location")
        {
            wars::GameOptions::assetLocation = *(argument + 1);
        } else
        {
            wars::GameOptions::assetLocation = *argument;
        }
    }
    
    try
    {
        wars::Game game;
        game.mainLoop();
    }
    catch(const std::runtime_error& e)
    {
        std::cerr << "[FATAL ERROR]: " << e.what() << '\n';
        return -1;
    }
    
    return 0;
}