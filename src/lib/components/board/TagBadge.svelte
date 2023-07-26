<script lang="ts">
    export let name: string;
    export let color: string;
</script>

<span style={`background-color: ${color}`} class="badge text-warning-50 mr-1"
    >{name}</span
>
