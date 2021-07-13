from setuptools_rust_starter import sleep_for, foo
import asyncio

async def test_python_sleep(loop):
    await asyncio.sleep(1)

async def test_rust_sleep_foo(loop):
    await foo()


async def test_rust_sleep(loop):
    await sleep_for(1)
