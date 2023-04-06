import { defineStore } from 'pinia'
import { Names } from './store-name'

export const useUserStore = defineStore(Names.USER, {
  state: () => {
    return {
      token: 'token'
    }
  },
  getters: {

  },
  actions: {

  }
})