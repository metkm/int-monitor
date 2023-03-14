<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

interface Table {
  row_count: number,
  rows: TableRow[]
}

interface TableRow {
  bandwidth: null | { inbound: number, outbound: number },
  create_timestamp: number,
  local_addr: string,
  local_port: number,
  remote_addr: string,
  remote_port: number,
  owning_pid: number,
  owning_module_info: null | Array<number>
}

const listener = ref<UnlistenFn | null>(null);
const table = ref<Table | null>();

onMounted(async () => {
  await listen<Table>("table-update", ({ payload }) => {
    table.value = payload;
  });

  // listener.value = await listen("table-update", (event) => {
  //   // let payload = event.payload as Table;

  //   // payload.rows.sort((a, b) => {
  //   //   if (a.bandwidth && !b.bandwidth) {
  //   //     // b is empty so it should come after
  //   //     return -1;
  //   //   }
  //   //   if (!a.bandwidth && b.bandwidth) {
  //   //     // a is empty so it should come first
  //   //     return 1;
  //   //   }

  //   //   if (a.bandwidth && b.bandwidth) {
  //   //     return b.bandwidth.inbound - a.bandwidth.inbound
  //   //   }

  //   //   return 0;
  //   // })

  //   // console.log(payload);
  //   // table.value = payload;
  // })

  invoke("init_process");
})

onUnmounted(() => listener.value?.())
</script>

<template>
  <div class="p-2">
    <table v-if="table" class="w-full rounded-lg overflow-hidden">
      <thead class="bg-neutral-200 dark:bg-neutral-800 border dark:border-neutral-800">
        <tr>
          <th class="p-2">Owning PID</th>
          <th class="p-2">Local Address</th>
          <th class="p-2">Local Port</th>
          <th class="p-2">Remote Address</th>
          <th class="p-2">Remote Port</th>

          <th class="p-2">Recv Bytes</th>
          <th class="p-2">Sent Bytes</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in table.rows" class="text-center border dark:border-neutral-800">
          <td class="p-2">{{ row.owning_pid }}</td>
          <td class="p-2">{{ row.local_addr }}</td>
          <td class="p-2">{{ row.local_port }}</td>
          <td class="p-2">{{ row.remote_addr }}</td>
          <td class="p-2">{{ row.remote_port }}</td>

          <template v-if="row.bandwidth">
            <td class="p-2">{{ row.bandwidth.inbound.toLocaleString() }}</td>
            <td class="p-2">{{ row.bandwidth.outbound.toLocaleString() }}</td>
          </template>
        </tr>
      </tbody>
    </table>
  </div>
</template>
