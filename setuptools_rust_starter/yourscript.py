import asyncio
import asyncclick as click
from setuptools_rust_starter import ExampleClass
from setuptools_rust_starter.ahoc import AhoCClass

@click.group()
def cli():
    pass


@click.command()
async def hello():
    """Example script."""
    click.echo('Hello World!')
    example = ExampleClass(value=12)

    await asyncio.sleep(3.0)
    click.echo(example.greetme())

cli.add_command(hello)

@click.command()
async def test():
    """Sample test"""
    click.echo('Running ahoc')
    ah = AhoCClass()

    ben = ah.test('Nobody likes maple in their apple flavored Snapple.')

    roy = ah.test_array(['abc', 'def'])

    # bob = ah.test_arraycount(['abc', 'def'])

    bob = [12,334]
    click.echo(f'I am now {ben} and of size {roy} and {bob}')


cli.add_command(test)

if __name__ == '__main__':
    cli(_anyio_backend="asyncio")
