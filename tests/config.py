"""
Vecstore API Test Configuration
Centralized configuration for all test scripts
"""

import argparse

# Environment configurations
ENVIRONMENTS = {
    "dev": {
        "api_key": "5aaae9932495ea2726d2318ac97dc36b6ecf55c05e65b7b0e102f61a8e433ca0",
        "base_url": "http://localhost:3000"
    },
    "prod": {
        "api_key": "f18718512e82150f1c813d750ebd0340a6198635a7b3650ed3077b69fc97725f",
        "base_url": "https://api.vecstore.app"
    }
}

def get_config(env="dev"):
    """
    Get configuration for specified environment

    Args:
        env (str): Environment name ("dev" or "prod")

    Returns:
        dict: Configuration dictionary with api_key and base_url
    """
    if env not in ENVIRONMENTS:
        raise ValueError(f"Invalid environment: {env}. Must be 'dev' or 'prod'")

    return ENVIRONMENTS[env]

def add_env_argument(parser):
    """
    Add standard --env argument to argparse parser

    Args:
        parser: argparse.ArgumentParser instance
    """
    parser.add_argument(
        "--env",
        choices=["dev", "prod"],
        default="dev",
        help="Environment: dev (localhost) or prod (api.vecstore.app)"
    )
