<script lang="ts">
    interface Props {
        name: string;
        color: string;
    }

    let { name, color }: Props = $props();
</script>

<span style={`background-color: ${color}`} class="badge text-warning-50 mr-1 mt-1">{name}</span>
