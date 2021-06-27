import click
from setuptools_rust_starter import ExampleClass


@click.group()
def cli():
    pass


@click.command()
def hello():
    """Example script."""
    click.echo('Hello World!')
    example = ExampleClass(value=12)

    click.echo(example.greetme())

cli.add_command(hello)




if __name__ == '__main__':
    cli()