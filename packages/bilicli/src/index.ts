#!/bin/env node

import { cac } from 'cac'
import { version } from '../package.json'
import { App, AppOptions, EditOptions, openEditor } from './app'

const cli = cac('bilicli')

cli
  .command('[room_id]', '输入房间号，打开直播间控制台')
  .option('--cookie <cookie>', '将你在B站登录的cookie粘贴到这里')
  .option('--uid <uid>', '你的B站UID')
  .option('--config [config]', '配置文件路径', { default: 'bilicli.config.js' })
  .action(async (roomId: string | undefined, options: AppOptions) => {
    const app = new App(roomId, options)
    await app.run()
    process.exit(0)
  })

cli
  .command('edit', 'Open editor to edit config')
  .option('--config [config]', 'Custom config file path', { default: 'bilicli.config.js' })
  .action((options: EditOptions) => {
    openEditor(options.config)
  })

cli.help()
cli.version(version)

cli.parse()

